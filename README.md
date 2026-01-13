# pyanylist

[![CI](https://github.com/ozonejunkieau/pyanylist/actions/workflows/ci.yml/badge.svg)](https://github.com/ozonejunkieau/pyanylist/actions/workflows/ci.yml)
[![PyPI](https://img.shields.io/pypi/v/pyanylist.svg)](https://pypi.org/project/pyanylist/)
[![Python Versions](https://img.shields.io/pypi/pyversions/pyanylist.svg)](https://pypi.org/project/pyanylist/)

Unofficial Python bindings for the [AnyList](https://www.anylist.com/) API, built with Rust and PyO3 for performance.

## Features

- **Shopping Lists**: Create, read, update, and delete shopping lists and items
- **Favourites**: Manage favourite/starter items
- **Recipes**: Full recipe CRUD with ingredient scaling
- **Meal Planning**: iCalendar integration for meal plan calendars
- **Real-time Sync**: WebSocket-based live updates
- **Token Persistence**: Save and restore authentication sessions

## Installation

```bash
uv add pyanylist
```

Or with pip:

```bash
pip install pyanylist
```

## Quick Start

```python
from pyanylist import AnyListClient

# Login with email and password
client = AnyListClient.login("your-email@example.com", "your-password")

# Get all shopping lists
lists = client.get_lists()
for lst in lists:
    print(f"{lst.name}: {len(lst.items)} items")

# Add an item to a list
item = client.add_item(lists[0].id, "Milk")
print(f"Added: {item.name}")

# Add item with details
item = client.add_item_with_details(
    lists[0].id,
    "Apples",
    quantity="6",
    details="Honeycrisp preferred",
    category="Produce"
)
```

## Authentication

### Login with credentials

```python
client = AnyListClient.login("email@example.com", "password")
```

### Save and restore session

```python
# Export tokens for persistence
tokens = client.export_tokens()

# Save to file, database, etc.
save_tokens(tokens.access_token, tokens.refresh_token, tokens.user_id)

# Later, restore from saved tokens
from pyanylist import SavedTokens

tokens = SavedTokens(
    access_token="...",
    refresh_token="...",
    user_id="...",
    is_premium_user=False
)
client = AnyListClient.from_tokens(tokens)
```

## Shopping Lists

```python
# Get all lists
lists = client.get_lists()

# Get a specific list
grocery_list = client.get_list_by_name("Groceries")
# or by ID
grocery_list = client.get_list_by_id("list-id-here")

# Create a new list
new_list = client.create_list("Weekly Shopping")

# Rename a list
client.rename_list(new_list.id, "Monthly Shopping")

# Delete a list
client.delete_list(new_list.id)
```

## List Items

```python
# Add a simple item
item = client.add_item(list_id, "Bread")

# Add item with details
item = client.add_item_with_details(
    list_id,
    "Chicken Breast",
    quantity="2 lbs",
    details="Boneless, skinless",
    category="Meat"
)

# Check/uncheck items
client.cross_off_item(list_id, item.id)
client.uncheck_item(list_id, item.id)

# Delete an item
client.delete_item(list_id, item.id)

# Clear all checked items
client.delete_all_crossed_off_items(list_id)
```

## Favourites

```python
# Get all favourite items
favourites = client.get_favourites()

# Get favourites organized by list
fav_lists = client.get_favourites_lists()
for fav_list in fav_lists:
    print(f"{fav_list.name}: {len(fav_list.items)} items")

# Add a favourite
fav = client.add_favourite("Coffee", category="Beverages")

# Remove a favourite
client.remove_favourite(fav_list.id, fav.id)
```

## Recipes

```python
from pyanylist import Ingredient

# Get all recipes
recipes = client.get_recipes()
for recipe in recipes:
    print(f"{recipe.name}: {len(recipe.ingredients)} ingredients")

# Get a specific recipe
recipe = client.get_recipe_by_name("Pasta Carbonara")

# Create a recipe
ingredients = [
    Ingredient("Spaghetti", quantity="400g"),
    Ingredient("Eggs", quantity="4"),
    Ingredient("Parmesan", quantity="100g", note="freshly grated"),
    Ingredient("Pancetta", quantity="200g"),
]
steps = [
    "Boil pasta according to package directions",
    "Fry pancetta until crispy",
    "Mix eggs and parmesan",
    "Combine everything off heat",
]
recipe = client.create_recipe("Carbonara", ingredients, steps)

# Add recipe ingredients to a shopping list
client.add_recipe_to_list(recipe.id, list_id)

# Scale recipe (e.g., double it)
client.add_recipe_to_list(recipe.id, list_id, scale_factor=2.0)

# Delete a recipe
client.delete_recipe(recipe.id)
```

## Meal Plan Calendar (iCalendar)

```python
# Enable iCalendar export
info = client.enable_icalendar()
print(f"Calendar URL: {info.url}")
# Use this URL in any calendar app (Google Calendar, Apple Calendar, etc.)

# Get existing calendar URL
url = client.get_icalendar_url()

# Disable iCalendar
client.disable_icalendar()
```

## Real-time Sync

```python
import time
from pyanylist import SyncEvent

# Start real-time sync
sync = client.start_realtime_sync()

# Poll for events
while sync.is_connected():
    events = sync.poll_events()
    for event in events:
        if event == SyncEvent.ShoppingListsChanged:
            print("Lists changed! Refreshing...")
            lists = client.get_lists()
        elif event == SyncEvent.RecipeDataChanged:
            print("Recipes changed!")
    time.sleep(1)

# Disconnect when done
sync.disconnect()
```

### Available Sync Events

- `ShoppingListsChanged` - Shopping lists modified
- `StarterListsChanged` - Favourites modified
- `RecipeDataChanged` - Recipes modified
- `MealPlanCalendarChanged` - Meal plan modified
- `AccountDeleted` - Account was deleted

## Error Handling

All methods raise `RuntimeError` on failure:

```python
try:
    client = AnyListClient.login("bad@email.com", "wrongpassword")
except RuntimeError as e:
    print(f"Login failed: {e}")

try:
    client.get_list_by_id("nonexistent-id")
except RuntimeError as e:
    print(f"List not found: {e}")
```

## Development

### Prerequisites

- Python 3.12+
- Rust 1.70+
- [uv](https://docs.astral.sh/uv/)
- protoc (Protocol Buffers compiler)
  - Ubuntu/Debian: `sudo apt-get install protobuf-compiler`
  - macOS: `brew install protobuf`

### Setup

```bash
# Clone the repository
git clone https://github.com/ozonejunkieau/pyanylist.git
cd pyanylist

# Install dependencies (includes dev dependencies)
uv sync

# Build and install in development mode
uv run maturin develop
```

### Running Tests

```bash
# Run unit tests (no credentials needed)
uv run pytest -v -m "not integration"

# Run all tests including integration (requires credentials)
export ANYLIST_EMAIL="your-email@example.com"
export ANYLIST_PASSWORD="your-password"
uv run pytest -v

# Run with coverage
uv run pytest --cov=tests --cov-report=term-missing
```

### Linting & Type Checking

```bash
# Check code style
uv run ruff check tests/

# Format code
uv run ruff format tests/

# Type check
uv run pyright tests/
```

## License

MIT License - see [LICENSE](LICENSE) for details.

## Disclaimer

This is an unofficial library and is not affiliated with or endorsed by AnyList. Use at your own risk and in accordance with AnyList's terms of service.
