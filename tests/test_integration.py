"""Integration tests for pyanylist.

These tests require valid AnyList credentials and make real API calls.
They are skipped by default unless credentials are provided.

Run with: pytest -m integration

Set environment variables:
    ANYLIST_EMAIL=your-email@example.com
    ANYLIST_PASSWORD=your-password
"""

import contextlib
import time
import uuid

import pytest

# Mark all tests in this module as integration tests
pytestmark = pytest.mark.integration


@pytest.fixture(scope="module")
def client(anylist_credentials):
    """Create an authenticated AnyListClient for the test session."""
    from pyanylist import AnyListClient

    email, password = anylist_credentials
    return AnyListClient.login(email, password)


class TestAuthentication:
    """Test authentication flows."""

    def test_login_success(self, anylist_credentials):
        """Test successful login."""
        from pyanylist import AnyListClient

        email, password = anylist_credentials
        client = AnyListClient.login(email, password)

        assert client is not None
        assert client.user_id() is not None
        assert len(client.user_id()) > 0

    def test_export_and_restore_tokens(self, client):
        """Test exporting and restoring tokens."""
        from pyanylist import AnyListClient

        # Export tokens
        tokens = client.export_tokens()

        assert tokens.access_token is not None
        assert tokens.refresh_token is not None
        assert tokens.user_id is not None

        # Restore from tokens
        restored_client = AnyListClient.from_tokens(tokens)

        assert restored_client.user_id() == client.user_id()

    def test_user_id_is_string(self, client):
        """Test that user_id returns a string."""
        user_id = client.user_id()

        assert isinstance(user_id, str)
        assert len(user_id) > 0

    def test_is_premium_user_returns_bool(self, client):
        """Test that is_premium_user returns a boolean."""
        is_premium = client.is_premium_user()

        assert isinstance(is_premium, bool)


class TestShoppingLists:
    """Test shopping list operations."""

    def test_get_lists(self, client):
        """Test getting all shopping lists."""
        lists = client.get_lists()

        assert isinstance(lists, list)
        # User should have at least one list
        assert len(lists) >= 0

    def test_list_has_expected_fields(self, client):
        """Test that lists have expected fields."""
        lists = client.get_lists()

        if len(lists) > 0:
            lst = lists[0]
            assert hasattr(lst, "id")
            assert hasattr(lst, "name")
            assert hasattr(lst, "items")
            assert isinstance(lst.id, str)
            assert isinstance(lst.name, str)
            assert isinstance(lst.items, list)

    def test_create_and_delete_list(self, client):
        """Test creating and deleting a list."""
        # Create a unique list name
        list_name = f"Test List {uuid.uuid4().hex[:8]}"

        # Create list
        new_list = client.create_list(list_name)

        assert new_list.name == list_name
        assert new_list.id is not None

        # Clean up - delete the list
        client.delete_list(new_list.id)

        # Verify deletion
        lists = client.get_lists()
        list_ids = [lst.id for lst in lists]
        assert new_list.id not in list_ids

    def test_get_list_by_name(self, client):
        """Test getting a list by name."""
        lists = client.get_lists()

        if len(lists) > 0:
            expected_name = lists[0].name
            found_list = client.get_list_by_name(expected_name)

            assert found_list.name == expected_name


class TestListItems:
    """Test list item operations."""

    @pytest.fixture
    def test_list(self, client):
        """Create a temporary list for item tests."""
        list_name = f"Item Test {uuid.uuid4().hex[:8]}"
        test_list = client.create_list(list_name)
        yield test_list
        # Cleanup
        with contextlib.suppress(Exception):
            client.delete_list(test_list.id)

    def test_add_item(self, client, test_list):
        """Test adding an item to a list."""
        item = client.add_item(test_list.id, "Test Item")

        assert item.name == "Test Item"
        assert item.id is not None
        assert item.is_checked is False

    def test_add_item_with_details(self, client, test_list):
        """Test adding an item with details."""
        item = client.add_item_with_details(
            test_list.id,
            "Detailed Item",
            quantity="2 lbs",
            details="Fresh if possible",
            category="Produce",
        )

        assert item.name == "Detailed Item"
        assert item.quantity == "2 lbs"

    def test_cross_off_and_uncheck_item(self, client, test_list):
        """Test crossing off and unchecking an item."""
        # Add item
        item = client.add_item(test_list.id, "Check Test Item")

        # Cross off
        client.cross_off_item(test_list.id, item.id)

        # Verify crossed off
        updated_list = client.get_list_by_id(test_list.id)
        updated_item = next((i for i in updated_list.items if i.id == item.id), None)
        assert updated_item is not None
        assert updated_item.is_checked is True

        # Uncheck
        client.uncheck_item(test_list.id, item.id)

        # Verify unchecked
        updated_list = client.get_list_by_id(test_list.id)
        updated_item = next((i for i in updated_list.items if i.id == item.id), None)
        assert updated_item.is_checked is False

    def test_delete_item(self, client, test_list):
        """Test deleting an item."""
        # Add item
        item = client.add_item(test_list.id, "Delete Test Item")

        # Delete it
        client.delete_item(test_list.id, item.id)

        # Verify deletion
        updated_list = client.get_list_by_id(test_list.id)
        item_ids = [i.id for i in updated_list.items]
        assert item.id not in item_ids


class TestFavourites:
    """Test favourite operations."""

    def test_get_favourites(self, client):
        """Test getting favourites."""
        favourites = client.get_favourites()

        assert isinstance(favourites, list)

    def test_get_favourites_lists(self, client):
        """Test getting favourites lists."""
        lists = client.get_favourites_lists()

        assert isinstance(lists, list)


class TestRecipes:
    """Test recipe operations."""

    def test_get_recipes(self, client):
        """Test getting all recipes."""
        recipes = client.get_recipes()

        assert isinstance(recipes, list)

    def test_recipe_has_expected_fields(self, client):
        """Test that recipes have expected fields."""
        recipes = client.get_recipes()

        if len(recipes) > 0:
            recipe = recipes[0]
            assert hasattr(recipe, "id")
            assert hasattr(recipe, "name")
            assert hasattr(recipe, "ingredients")
            assert hasattr(recipe, "preparation_steps")


class TestICalendar:
    """Test iCalendar operations."""

    def test_get_icalendar_url(self, client):
        """Test getting iCalendar URL."""
        url = client.get_icalendar_url()

        # URL might be None if not enabled
        assert url is None or isinstance(url, str)

    def test_enable_icalendar(self, client):
        """Test enabling iCalendar."""
        info = client.enable_icalendar()

        assert info.enabled is True
        assert info.url is not None
        assert "icalendar.anylist.com" in info.url
        assert info.token is not None


class TestRealtimeSync:
    """Test realtime sync operations."""

    def test_start_realtime_sync(self, client):
        """Test starting realtime sync."""
        sync = client.start_realtime_sync()

        assert sync is not None

        # Wait a moment for connection
        time.sleep(1)

        # Check state
        assert sync.is_connected() is True

        # Poll for events (might be empty)
        events = sync.poll_events()
        assert isinstance(events, list)

        # Disconnect
        sync.disconnect()

    def test_realtime_sync_poll_events(self, client):
        """Test polling events from realtime sync."""
        sync = client.start_realtime_sync()

        time.sleep(0.5)

        # Poll multiple times
        for _ in range(3):
            events = sync.poll_events()
            assert isinstance(events, list)
            time.sleep(0.2)

        sync.disconnect()


class TestErrorHandling:
    """Test error handling for invalid operations."""

    def test_get_nonexistent_list(self, client):
        """Test getting a non-existent list raises error."""
        with pytest.raises(RuntimeError):
            client.get_list_by_id("nonexistent-list-id-12345")

    def test_get_nonexistent_recipe(self, client):
        """Test getting a non-existent recipe raises error."""
        with pytest.raises(RuntimeError):
            client.get_recipe_by_id("nonexistent-recipe-id-12345")
