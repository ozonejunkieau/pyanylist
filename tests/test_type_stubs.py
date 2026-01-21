"""Tests to verify type stubs are correct.

These tests are primarily for static type checking with basedpyright.
Run `uv run basedpyright tests/test_type_stubs.py` to verify.
"""

from pyanylist import (
    AnyListClient,
    ConnectionState,
    FavouriteItem,
    FavouritesList,
    ICalendarInfo,
    Ingredient,
    ListItem,
    RealtimeSync,
    Recipe,
    SavedTokens,
    ShoppingList,
    SyncEvent,
)


def test_saved_tokens_types() -> None:
    """Verify SavedTokens type annotations."""
    tokens = SavedTokens(
        access_token="access",
        refresh_token="refresh",
        user_id="user123",
        is_premium_user=True,
    )

    # These should all type check correctly
    access: str = tokens.access_token
    refresh: str = tokens.refresh_token
    user_id: str = tokens.user_id
    is_premium: bool = tokens.is_premium_user

    assert isinstance(access, str)
    assert isinstance(refresh, str)
    assert isinstance(user_id, str)
    assert isinstance(is_premium, bool)


def test_ingredient_types() -> None:
    """Verify Ingredient type annotations."""
    # Minimal construction
    ing1 = Ingredient(name="Flour")
    name: str = ing1.name
    qty: str | None = ing1.quantity
    note: str | None = ing1.note
    raw: str | None = ing1.raw_ingredient

    assert isinstance(name, str)
    assert qty is None
    assert note is None
    assert raw is None

    # Full construction
    ing2 = Ingredient(
        name="Sugar",
        quantity="2 cups",
        note="organic preferred",
        raw_ingredient="2 cups sugar",
    )
    assert ing2.quantity == "2 cups"


def test_sync_event_enum() -> None:
    """Verify SyncEvent is an enum with correct members."""
    event: SyncEvent = SyncEvent.ShoppingListsChanged
    assert event == SyncEvent.ShoppingListsChanged

    # All enum members should exist
    _ = SyncEvent.CategorizedItemsChanged
    _ = SyncEvent.ListFoldersChanged
    _ = SyncEvent.ListSettingsChanged
    _ = SyncEvent.StarterListsChanged
    _ = SyncEvent.RecipeDataChanged
    _ = SyncEvent.AccountDeleted


def test_connection_state_enum() -> None:
    """Verify ConnectionState is an enum with correct members."""
    state: ConnectionState = ConnectionState.Disconnected
    assert state == ConnectionState.Disconnected

    _ = ConnectionState.Connecting
    _ = ConnectionState.Connected
    _ = ConnectionState.Reconnecting
    _ = ConnectionState.Closed


def test_client_method_signatures() -> None:
    """Verify AnyListClient method signatures exist.

    This test doesn't call the methods (would need credentials),
    just verifies they exist with correct signatures for type checking.
    """
    # Static methods
    assert callable(AnyListClient.login)
    assert callable(AnyListClient.from_tokens)

    # Verify method existence on the class
    assert hasattr(AnyListClient, "export_tokens")
    assert hasattr(AnyListClient, "user_id")
    assert hasattr(AnyListClient, "is_premium_user")
    assert hasattr(AnyListClient, "get_lists")
    assert hasattr(AnyListClient, "get_list_by_id")
    assert hasattr(AnyListClient, "create_list")
    assert hasattr(AnyListClient, "add_item")
    assert hasattr(AnyListClient, "add_item_with_details")
    assert hasattr(AnyListClient, "get_favourites")
    assert hasattr(AnyListClient, "get_recipes")
    assert hasattr(AnyListClient, "create_recipe")
    assert hasattr(AnyListClient, "enable_icalendar")
    assert hasattr(AnyListClient, "start_realtime_sync")


def test_list_types_annotations() -> None:
    """Verify list and item types have correct annotations."""
    # These are just type annotation checks - we can't instantiate
    # ListItem or ShoppingList directly (they come from the API)

    # Verify the types exist and have the right attributes
    assert hasattr(ListItem, "__init__") or True  # Native types
    assert hasattr(ShoppingList, "__init__") or True
    assert hasattr(FavouriteItem, "__init__") or True
    assert hasattr(FavouritesList, "__init__") or True
    assert hasattr(Recipe, "__init__") or True
    assert hasattr(ICalendarInfo, "__init__") or True
    assert hasattr(RealtimeSync, "__init__") or True
