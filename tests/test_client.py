"""Test AnyListClient methods and error handling."""

import pytest


class TestClientStaticMethods:
    """Test AnyListClient static methods."""

    def test_login_method_exists(self):
        """Test that login static method exists."""
        from pyanylist import AnyListClient

        assert hasattr(AnyListClient, "login")

    def test_from_tokens_method_exists(self):
        """Test that from_tokens static method exists."""
        from pyanylist import AnyListClient

        assert hasattr(AnyListClient, "from_tokens")


class TestClientLoginErrors:
    """Test AnyListClient login error handling."""

    def test_login_invalid_credentials(self):
        """Test that login with invalid credentials raises error."""
        from pyanylist import AnyListClient

        with pytest.raises(RuntimeError) as exc_info:
            AnyListClient.login("invalid@example.com", "wrongpassword")

        # Should contain some error message
        assert "failed" in str(exc_info.value).lower() or "error" in str(exc_info.value).lower()

    def test_login_empty_email(self):
        """Test that login with empty email raises error."""
        from pyanylist import AnyListClient

        with pytest.raises((RuntimeError, ValueError)):
            AnyListClient.login("", "password")

    def test_login_empty_password(self):
        """Test that login with empty password raises error."""
        from pyanylist import AnyListClient

        with pytest.raises((RuntimeError, ValueError)):
            AnyListClient.login("test@example.com", "")


class TestClientFromTokensErrors:
    """Test AnyListClient from_tokens error handling."""

    def test_from_tokens_creates_client(self):
        """Test that from_tokens creates a client (validation is lazy)."""
        from pyanylist import AnyListClient, SavedTokens

        tokens = SavedTokens(
            access_token="invalid_token",
            refresh_token="invalid_refresh",
            user_id="test_user",
            is_premium_user=False,
        )

        # from_tokens should succeed - validation happens on API calls
        client = AnyListClient.from_tokens(tokens)
        assert client is not None
        assert client.user_id() == "test_user"

    def test_from_tokens_invalid_api_call_fails(self):
        """Test that API calls with invalid tokens fail."""
        from pyanylist import AnyListClient, SavedTokens

        tokens = SavedTokens(
            access_token="invalid_token",
            refresh_token="invalid_refresh",
            user_id="invalid_user",
            is_premium_user=False,
        )

        client = AnyListClient.from_tokens(tokens)

        # API call should fail with invalid tokens
        with pytest.raises(RuntimeError):
            client.get_lists()


class TestClientMethodSignatures:
    """Test that AnyListClient has expected methods with correct signatures."""

    def test_has_list_methods(self):
        """Test that client has list-related methods."""
        from pyanylist import AnyListClient

        expected_methods = [
            "get_lists",
            "get_list_by_id",
            "get_list_by_name",
            "create_list",
            "delete_list",
            "rename_list",
        ]

        for method in expected_methods:
            assert hasattr(AnyListClient, method), f"Missing method: {method}"

    def test_has_item_methods(self):
        """Test that client has item-related methods."""
        from pyanylist import AnyListClient

        expected_methods = [
            "add_item",
            "add_item_with_details",
            "delete_item",
            "cross_off_item",
            "uncheck_item",
            "delete_all_crossed_off_items",
        ]

        for method in expected_methods:
            assert hasattr(AnyListClient, method), f"Missing method: {method}"

    def test_has_favourite_methods(self):
        """Test that client has favourite-related methods."""
        from pyanylist import AnyListClient

        expected_methods = [
            "get_favourites",
            "get_favourites_lists",
            "add_favourite",
            "remove_favourite",
        ]

        for method in expected_methods:
            assert hasattr(AnyListClient, method), f"Missing method: {method}"

    def test_has_recipe_methods(self):
        """Test that client has recipe-related methods."""
        from pyanylist import AnyListClient

        expected_methods = [
            "get_recipes",
            "get_recipe_by_id",
            "get_recipe_by_name",
            "create_recipe",
            "update_recipe",
            "delete_recipe",
            "add_recipe_to_list",
        ]

        for method in expected_methods:
            assert hasattr(AnyListClient, method), f"Missing method: {method}"

    def test_has_icalendar_methods(self):
        """Test that client has iCalendar-related methods."""
        from pyanylist import AnyListClient

        expected_methods = [
            "enable_icalendar",
            "disable_icalendar",
            "get_icalendar_url",
        ]

        for method in expected_methods:
            assert hasattr(AnyListClient, method), f"Missing method: {method}"

    def test_has_realtime_methods(self):
        """Test that client has realtime sync methods."""
        from pyanylist import AnyListClient

        assert hasattr(AnyListClient, "start_realtime_sync")

    def test_has_token_methods(self):
        """Test that client has token-related methods."""
        from pyanylist import AnyListClient

        expected_methods = [
            "export_tokens",
            "user_id",
            "is_premium_user",
        ]

        for method in expected_methods:
            assert hasattr(AnyListClient, method), f"Missing method: {method}"


class TestIngredientConstruction:
    """Test Ingredient construction for recipe creation."""

    def test_ingredient_list_creation(self, sample_ingredients):
        """Test creating a list of ingredients."""
        assert len(sample_ingredients) == 4
        assert sample_ingredients[0].name == "Flour"
        assert sample_ingredients[0].quantity == "2 cups"
        assert sample_ingredients[1].note == "granulated"
        assert sample_ingredients[3].quantity is None

    def test_ingredient_can_be_used_in_list(self):
        """Test that ingredients can be stored in a Python list."""
        from pyanylist import Ingredient

        ingredients = []
        ingredients.append(Ingredient("Item 1"))
        ingredients.append(Ingredient("Item 2", quantity="1"))

        assert len(ingredients) == 2
        assert ingredients[0].name == "Item 1"
        assert ingredients[1].quantity == "1"
