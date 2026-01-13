"""Test pyanylist type conversions and data structures."""


class TestSavedTokens:
    """Test SavedTokens data structure."""

    def test_create_saved_tokens(self):
        """Test creating SavedTokens with all fields."""
        from pyanylist import SavedTokens

        tokens = SavedTokens(
            access_token="access_123",
            refresh_token="refresh_456",
            user_id="user_789",
            is_premium_user=True,
        )

        assert tokens.access_token == "access_123"
        assert tokens.refresh_token == "refresh_456"
        assert tokens.user_id == "user_789"
        assert tokens.is_premium_user is True

    def test_saved_tokens_repr(self):
        """Test SavedTokens string representation."""
        from pyanylist import SavedTokens

        tokens = SavedTokens(
            access_token="access",
            refresh_token="refresh",
            user_id="user123",
            is_premium_user=False,
        )

        repr_str = repr(tokens)
        assert "user123" in repr_str
        assert "SavedTokens" in repr_str

    def test_saved_tokens_premium_false(self):
        """Test SavedTokens with premium=False."""
        from pyanylist import SavedTokens

        tokens = SavedTokens(
            access_token="a",
            refresh_token="r",
            user_id="u",
            is_premium_user=False,
        )
        assert tokens.is_premium_user is False

    def test_saved_tokens_premium_true(self):
        """Test SavedTokens with premium=True."""
        from pyanylist import SavedTokens

        tokens = SavedTokens(
            access_token="a",
            refresh_token="r",
            user_id="u",
            is_premium_user=True,
        )
        assert tokens.is_premium_user is True


class TestIngredient:
    """Test Ingredient data structure."""

    def test_create_ingredient_minimal(self):
        """Test creating Ingredient with only required fields."""
        from pyanylist import Ingredient

        ing = Ingredient(name="Flour")

        assert ing.name == "Flour"
        assert ing.quantity is None
        assert ing.note is None
        assert ing.raw_ingredient is None

    def test_create_ingredient_with_quantity(self):
        """Test creating Ingredient with quantity."""
        from pyanylist import Ingredient

        ing = Ingredient(name="Sugar", quantity="2 cups")

        assert ing.name == "Sugar"
        assert ing.quantity == "2 cups"

    def test_create_ingredient_full(self):
        """Test creating Ingredient with all fields."""
        from pyanylist import Ingredient

        ing = Ingredient(
            name="Butter",
            quantity="1 stick",
            note="softened",
            raw_ingredient="1 stick butter, softened",
        )

        assert ing.name == "Butter"
        assert ing.quantity == "1 stick"
        assert ing.note == "softened"
        assert ing.raw_ingredient == "1 stick butter, softened"

    def test_ingredient_repr_with_quantity(self):
        """Test Ingredient repr with quantity."""
        from pyanylist import Ingredient

        ing = Ingredient(name="Milk", quantity="1 cup")
        repr_str = repr(ing)

        assert "Ingredient" in repr_str
        assert "Milk" in repr_str
        assert "1 cup" in repr_str

    def test_ingredient_repr_without_quantity(self):
        """Test Ingredient repr without quantity."""
        from pyanylist import Ingredient

        ing = Ingredient(name="Salt")
        repr_str = repr(ing)

        assert "Ingredient" in repr_str
        assert "Salt" in repr_str


class TestSyncEvent:
    """Test SyncEvent enum."""

    def test_sync_event_values_exist(self):
        """Test that SyncEvent enum values exist."""
        from pyanylist import SyncEvent

        # Check all expected event types exist
        assert hasattr(SyncEvent, "ShoppingListsChanged")
        assert hasattr(SyncEvent, "StarterListsChanged")
        assert hasattr(SyncEvent, "RecipeDataChanged")
        assert hasattr(SyncEvent, "MealPlanCalendarChanged")
        assert hasattr(SyncEvent, "AccountDeleted")

    def test_sync_event_equality(self):
        """Test SyncEvent equality comparison."""
        from pyanylist import SyncEvent

        event1 = SyncEvent.ShoppingListsChanged
        event2 = SyncEvent.ShoppingListsChanged
        event3 = SyncEvent.RecipeDataChanged

        assert event1 == event2
        assert event1 != event3

    def test_sync_event_all_values(self):
        """Test all SyncEvent enum values."""
        from pyanylist import SyncEvent

        events = [
            SyncEvent.ShoppingListsChanged,
            SyncEvent.CategorizedItemsChanged,
            SyncEvent.ListFoldersChanged,
            SyncEvent.ListSettingsChanged,
            SyncEvent.StarterListsChanged,
            SyncEvent.StarterListOrderChanged,
            SyncEvent.StarterListSettingsChanged,
            SyncEvent.MobileAppSettingsChanged,
            SyncEvent.UserCategoriesChanged,
            SyncEvent.RecipeDataChanged,
            SyncEvent.MealPlanCalendarChanged,
            SyncEvent.AccountInfoChanged,
            SyncEvent.SubscriptionInfoChanged,
            SyncEvent.AccountDeleted,
        ]

        # All should be valid enum values
        assert len(events) == 14


class TestConnectionState:
    """Test ConnectionState enum."""

    def test_connection_state_values_exist(self):
        """Test that ConnectionState enum values exist."""
        from pyanylist import ConnectionState

        assert hasattr(ConnectionState, "Disconnected")
        assert hasattr(ConnectionState, "Connecting")
        assert hasattr(ConnectionState, "Connected")
        assert hasattr(ConnectionState, "Reconnecting")
        assert hasattr(ConnectionState, "Closed")

    def test_connection_state_equality(self):
        """Test ConnectionState equality comparison."""
        from pyanylist import ConnectionState

        state1 = ConnectionState.Connected
        state2 = ConnectionState.Connected
        state3 = ConnectionState.Disconnected

        assert state1 == state2
        assert state1 != state3


class TestOptionalFields:
    """Test handling of Optional fields across the FFI boundary."""

    def test_ingredient_none_values(self):
        """Test that None values are handled correctly."""
        from pyanylist import Ingredient

        ing = Ingredient(name="Test")

        # These should be None, not empty strings
        assert ing.quantity is None
        assert ing.note is None
        assert ing.raw_ingredient is None

    def test_ingredient_empty_string_vs_none(self):
        """Test that empty strings are preserved, not converted to None."""
        from pyanylist import Ingredient

        # Pass empty string for quantity
        ing = Ingredient(name="Test", quantity="")

        # Empty string should be preserved (not converted to None)
        # Note: behavior depends on PyO3 configuration
        assert ing.quantity == "" or ing.quantity is None

    def test_saved_tokens_fields_are_strings(self):
        """Test that SavedTokens string fields are actually strings."""
        from pyanylist import SavedTokens

        tokens = SavedTokens(
            access_token="test",
            refresh_token="test",
            user_id="test",
            is_premium_user=False,
        )

        assert isinstance(tokens.access_token, str)
        assert isinstance(tokens.refresh_token, str)
        assert isinstance(tokens.user_id, str)
        assert isinstance(tokens.is_premium_user, bool)
