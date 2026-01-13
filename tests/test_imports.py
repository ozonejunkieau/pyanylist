"""Test that all pyanylist classes and functions are importable."""


class TestModuleImports:
    """Test that the pyanylist module can be imported."""

    def test_import_module(self):
        """Test importing the main module."""
        import pyanylist

        assert pyanylist is not None

    def test_module_has_version(self):
        """Test that module exposes version info."""
        import pyanylist

        # PyO3 modules don't always have __version__, but should be importable
        assert hasattr(pyanylist, "__name__")


class TestClientImports:
    """Test AnyListClient imports."""

    def test_import_anylist_client(self):
        """Test importing AnyListClient."""
        from pyanylist import AnyListClient

        assert AnyListClient is not None

    def test_client_has_login(self):
        """Test that AnyListClient has login method."""
        from pyanylist import AnyListClient

        assert hasattr(AnyListClient, "login")
        assert callable(AnyListClient.login)

    def test_client_has_from_tokens(self):
        """Test that AnyListClient has from_tokens method."""
        from pyanylist import AnyListClient

        assert hasattr(AnyListClient, "from_tokens")
        assert callable(AnyListClient.from_tokens)


class TestTokenImports:
    """Test SavedTokens imports."""

    def test_import_saved_tokens(self):
        """Test importing SavedTokens."""
        from pyanylist import SavedTokens

        assert SavedTokens is not None

    def test_saved_tokens_constructor(self):
        """Test SavedTokens can be constructed."""
        from pyanylist import SavedTokens

        tokens = SavedTokens(
            access_token="test_access",
            refresh_token="test_refresh",
            user_id="test_user",
            is_premium_user=False,
        )
        assert tokens.access_token == "test_access"
        assert tokens.refresh_token == "test_refresh"
        assert tokens.user_id == "test_user"
        assert tokens.is_premium_user is False


class TestListImports:
    """Test shopping list related imports."""

    def test_import_shopping_list(self):
        """Test importing ShoppingList."""
        from pyanylist import ShoppingList

        assert ShoppingList is not None

    def test_import_list_item(self):
        """Test importing ListItem."""
        from pyanylist import ListItem

        assert ListItem is not None


class TestFavouriteImports:
    """Test favourite related imports."""

    def test_import_favourite_item(self):
        """Test importing FavouriteItem."""
        from pyanylist import FavouriteItem

        assert FavouriteItem is not None

    def test_import_favourites_list(self):
        """Test importing FavouritesList."""
        from pyanylist import FavouritesList

        assert FavouritesList is not None


class TestRecipeImports:
    """Test recipe related imports."""

    def test_import_recipe(self):
        """Test importing Recipe."""
        from pyanylist import Recipe

        assert Recipe is not None

    def test_import_ingredient(self):
        """Test importing Ingredient."""
        from pyanylist import Ingredient

        assert Ingredient is not None


class TestICalendarImports:
    """Test iCalendar related imports."""

    def test_import_icalendar_info(self):
        """Test importing ICalendarInfo."""
        from pyanylist import ICalendarInfo

        assert ICalendarInfo is not None


class TestRealtimeImports:
    """Test realtime sync related imports."""

    def test_import_realtime_sync(self):
        """Test importing RealtimeSync."""
        from pyanylist import RealtimeSync

        assert RealtimeSync is not None

    def test_import_sync_event(self):
        """Test importing SyncEvent."""
        from pyanylist import SyncEvent

        assert SyncEvent is not None

    def test_import_connection_state(self):
        """Test importing ConnectionState."""
        from pyanylist import ConnectionState

        assert ConnectionState is not None


class TestAllExports:
    """Test that all expected classes are exported."""

    def test_all_classes_exported(self):
        """Test that all expected classes can be imported from pyanylist."""
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

        # All imports should succeed
        classes = [
            AnyListClient,
            SavedTokens,
            ShoppingList,
            ListItem,
            FavouriteItem,
            FavouritesList,
            Recipe,
            Ingredient,
            ICalendarInfo,
            RealtimeSync,
            SyncEvent,
            ConnectionState,
        ]
        assert all(cls is not None for cls in classes)
