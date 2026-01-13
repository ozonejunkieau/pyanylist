"""Pytest configuration and fixtures for pyanylist tests."""

import os

import pytest


def pytest_configure(config):
    """Configure pytest with custom markers."""
    config.addinivalue_line(
        "markers", "integration: marks tests as integration tests requiring credentials"
    )


@pytest.fixture
def anylist_credentials():
    """Get AnyList credentials from environment variables.

    Returns tuple of (email, password) or skips if not available.
    """
    email = os.environ.get("ANYLIST_EMAIL")
    password = os.environ.get("ANYLIST_PASSWORD")

    if not email or not password:
        pytest.skip("ANYLIST_EMAIL and ANYLIST_PASSWORD environment variables required")

    return email, password


@pytest.fixture
def sample_ingredient():
    """Create a sample Ingredient for testing."""
    from pyanylist import Ingredient

    return Ingredient(
        name="Test Ingredient",
        quantity="2 cups",
        note="Test note",
    )


@pytest.fixture
def sample_ingredients():
    """Create a list of sample Ingredients for testing."""
    from pyanylist import Ingredient

    return [
        Ingredient("Flour", quantity="2 cups"),
        Ingredient("Sugar", quantity="1 cup", note="granulated"),
        Ingredient("Eggs", quantity="3"),
        Ingredient("Butter"),
    ]
