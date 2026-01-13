"""Pytest configuration and fixtures for pyanylist tests."""

import os
import subprocess
import sys

import pytest


def pytest_configure(config):
    """Configure pytest with custom markers and ensure fresh build."""
    config.addinivalue_line(
        "markers", "integration: marks tests as integration tests requiring credentials"
    )

    # Ensure native module is built from current source before running tests.
    # This prevents stale cached binaries from masking source code issues.
    # Set SKIP_MATURIN_BUILD=1 to skip this (e.g., in CI after explicit build step).
    if not os.environ.get("SKIP_MATURIN_BUILD"):
        result = subprocess.run(
            [sys.executable, "-m", "maturin", "develop"],
            capture_output=True,
            text=True,
        )
        if result.returncode != 0:
            raise RuntimeError(f"Failed to build native module:\n{result.stderr}\n{result.stdout}")


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
