//! Flutter-inspired Gesture Arena (手势竞技场) for Pomelo UI.
//!
//! Provides a centralized arbitration mechanism for competing gesture recognizers
//! tracking the same pointer (e.g. Tap vs Drag vs Swipe vs outer scroll containers).

use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};

use iced::Point;

/// Unique identifier for an input pointer (touch finger or mouse button).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PointerId(pub usize);

impl PointerId {
    /// Pointer ID for mouse cursor left button.
    pub const MOUSE_LEFT: PointerId = PointerId(usize::MAX);

    /// Constructs a [`PointerId`] from an iced touch finger ID.
    pub fn touch(id: u64) -> Self {
        Self(id as usize)
    }
}

/// Identifier for a gesture recognizer participant in the arena.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MemberId(pub usize);

static NEXT_MEMBER_ID: AtomicUsize = AtomicUsize::new(1);

impl MemberId {
    /// Generates a globally unique member identifier.
    pub fn next() -> Self {
        Self(NEXT_MEMBER_ID.fetch_add(1, Ordering::Relaxed))
    }
}

/// Declaration of a recognizer's disposition in the arena.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GestureDisposition {
    /// The recognizer satisfies its criteria (e.g. displacement > slop) and claims victory.
    Accepted,
    /// The recognizer gives up (e.g. pointer moved outside bounds or exceeded timeout).
    Rejected,
}

/// A candidate participant in the gesture arena.
pub trait GestureArenaMember {
    /// Called when this member wins the arena and receives exclusive gesture ownership.
    fn accept_gesture(&mut self, pointer: PointerId);

    /// Called when this member is rejected by the arena and must yield gracefully.
    fn reject_gesture(&mut self, pointer: PointerId);
}

/// State of an individual gesture arena for a given pointer.
#[derive(Debug, Clone)]
pub struct ArenaEntry {
    /// Ordered list of candidate members participating in the arena.
    pub members: Vec<MemberId>,
    /// Whether this arena is open for new participants.
    pub is_open: bool,
    /// The resolved winner of the arena, if any.
    pub winner: Option<MemberId>,
    /// Whether any member in this arena has claimed exclusive ownership (capturing events).
    pub is_claimed: bool,
    /// The initial press location for this pointer.
    pub initial_point: Point,
}

/// Central gesture arena manager.
#[derive(Debug, Default, Clone)]
pub struct GestureArenaManager {
    arenas: HashMap<PointerId, ArenaEntry>,
}

impl GestureArenaManager {
    /// Creates a new, empty [`GestureArenaManager`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Opens an arena for `pointer` starting at `point`.
    pub fn open_arena(&mut self, pointer: PointerId, point: Point) {
        self.arenas.entry(pointer).or_insert_with(|| ArenaEntry {
            members: Vec::new(),
            is_open: true,
            winner: None,
            is_claimed: false,
            initial_point: point,
        });
    }

    /// Adds a candidate `member` to the arena for `pointer`.
    pub fn add_member(&mut self, pointer: PointerId, member: MemberId) {
        if let Some(entry) = self.arenas.get_mut(&pointer) {
            if entry.is_open && !entry.members.contains(&member) {
                entry.members.push(member);
            }
        }
    }

    /// Closes registration for `pointer`, preventing further recognizers from joining.
    pub fn close_arena(&mut self, pointer: PointerId) {
        if let Some(entry) = self.arenas.get_mut(&pointer) {
            entry.is_open = false;
        }
    }

    /// Marks the pointer as claimed exclusively, meaning child recognizers own it
    /// and outer containers (like [`Scrollable`]) should not intercept.
    pub fn claim(&mut self, pointer: PointerId) {
        if let Some(entry) = self.arenas.get_mut(&pointer) {
            entry.is_claimed = true;
        }
    }

    /// Returns whether the pointer is claimed.
    pub fn is_claimed(&self, pointer: PointerId) -> bool {
        self.arenas.get(&pointer).map_or(false, |e| e.is_claimed)
    }

    /// Resolves the disposition of `member` in the arena for `pointer`.
    ///
    /// If `Accepted`:
    /// Declares `member` the winner, closes the arena, and rejects all other candidate members.
    ///
    /// If `Rejected`:
    /// Removes `member` from candidates. If only 1 candidate remains and the arena is closed,
    /// that remaining member automatically wins.
    pub fn resolve(
        &mut self,
        pointer: PointerId,
        member: MemberId,
        disposition: GestureDisposition,
    ) -> Option<MemberId> {
        let entry = self.arenas.get_mut(&pointer)?;

        if entry.winner.is_some() {
            return entry.winner;
        }

        match disposition {
            GestureDisposition::Accepted => {
                entry.winner = Some(member);
                entry.is_open = false;
                Some(member)
            }
            GestureDisposition::Rejected => {
                entry.members.retain(|&m| m != member);
                if !entry.is_open && entry.members.len() == 1 {
                    entry.winner = Some(entry.members[0]);
                    entry.winner
                } else {
                    None
                }
            }
        }
    }

    /// Returns whether `member` is the resolved winner for `pointer`.
    pub fn is_member_winner(&self, pointer: PointerId, member: MemberId) -> bool {
        self.winner(pointer) == Some(member)
    }

    /// Returns the resolved winner of the arena for `pointer`, if any.
    pub fn winner(&self, pointer: PointerId) -> Option<MemberId> {
        self.arenas.get(&pointer).and_then(|e| e.winner)
    }

    /// Resolves any undecided arena on pointer release (Sweep).
    ///
    /// If no member called `Accepted`, the first registered member wins (default priority).
    pub fn sweep(&mut self, pointer: PointerId) -> Option<MemberId> {
        let entry = self.arenas.get_mut(&pointer)?;
        if entry.winner.is_none() && !entry.members.is_empty() {
            entry.winner = Some(entry.members[0]);
        }
        entry.winner
    }

    /// Closes and removes the arena for `pointer`.
    pub fn release(&mut self, pointer: PointerId) {
        self.arenas.remove(&pointer);
    }
}

thread_local! {
    static ARENA_CONTEXT: RefCell<Option<GestureArenaManager>> = const { RefCell::new(None) };
}

/// Runs a closure with the active thread-local arena context.
pub fn with_arena<R>(f: impl FnOnce(&mut GestureArenaManager) -> R) -> Option<R> {
    ARENA_CONTEXT.with(|ctx| {
        let mut borrow = ctx.borrow_mut();
        borrow.as_mut().map(f)
    })
}

/// Sets the active thread-local arena context, returning the previous context.
pub fn set_arena_context(arena: Option<GestureArenaManager>) -> Option<GestureArenaManager> {
    ARENA_CONTEXT.with(|ctx| ctx.replace(arena))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arena_lifecycle_open_add_claim_and_accept() {
        let mut manager = GestureArenaManager::new();
        let ptr = PointerId::touch(1);
        let m1 = MemberId::next();
        let m2 = MemberId::next();

        manager.open_arena(ptr, Point::new(10.0, 20.0));
        manager.add_member(ptr, m1);
        manager.add_member(ptr, m2);
        assert!(!manager.is_claimed(ptr));

        manager.claim(ptr);
        assert!(manager.is_claimed(ptr));

        // m1 accepts
        let winner = manager.resolve(ptr, m1, GestureDisposition::Accepted);
        assert_eq!(winner, Some(m1));
        assert_eq!(manager.winner(ptr), Some(m1));
        assert!(manager.is_member_winner(ptr, m1));
        assert!(!manager.is_member_winner(ptr, m2));

        manager.release(ptr);
        assert_eq!(manager.winner(ptr), None);
    }

    #[test]
    fn arena_rejection_and_auto_win() {
        let mut manager = GestureArenaManager::new();
        let ptr = PointerId::touch(2);
        let m1 = MemberId::next();
        let m2 = MemberId::next();

        manager.open_arena(ptr, Point::ORIGIN);
        manager.add_member(ptr, m1);
        manager.add_member(ptr, m2);
        manager.close_arena(ptr);

        // m1 rejects -> m2 is the only remaining member, automatically wins
        let winner = manager.resolve(ptr, m1, GestureDisposition::Rejected);
        assert_eq!(winner, Some(m2));
        assert_eq!(manager.winner(ptr), Some(m2));
    }

    #[test]
    fn arena_sweep_resolves_first_member() {
        let mut manager = GestureArenaManager::new();
        let ptr = PointerId::touch(3);
        let m1 = MemberId::next();
        let m2 = MemberId::next();

        manager.open_arena(ptr, Point::ORIGIN);
        manager.add_member(ptr, m1);
        manager.add_member(ptr, m2);

        // No one accepted or rejected. On sweep (release):
        let winner = manager.sweep(ptr);
        assert_eq!(winner, Some(m1));
    }

    #[test]
    fn thread_local_context_works() {
        assert!(with_arena(|_| ()).is_none());

        let manager = GestureArenaManager::new();
        let prev = set_arena_context(Some(manager));
        assert!(prev.is_none());

        let claimed = with_arena(|arena| {
            let ptr = PointerId::touch(99);
            arena.open_arena(ptr, Point::ORIGIN);
            arena.claim(ptr);
            arena.is_claimed(ptr)
        });
        assert_eq!(claimed, Some(true));

        let restored = set_arena_context(None);
        assert!(restored.is_some());
        assert!(with_arena(|_| ()).is_none());
    }
}

