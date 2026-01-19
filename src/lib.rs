//! Python bindings for anylist_rs
//!
//! This crate provides Python bindings for the anylist_rs library using PyO3.

// Allow clippy lints triggered by pyo3 macro-generated code
#![allow(clippy::useless_conversion)]
#![allow(clippy::uninlined_format_args)]

use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Python-compatible saved tokens for session persistence
#[pyclass]
#[derive(Clone)]
pub struct SavedTokens {
    #[pyo3(get, set)]
    pub access_token: String,
    #[pyo3(get, set)]
    pub refresh_token: String,
    #[pyo3(get, set)]
    pub user_id: String,
    #[pyo3(get, set)]
    pub is_premium_user: bool,
}

#[pymethods]
impl SavedTokens {
    #[new]
    fn new(
        access_token: String,
        refresh_token: String,
        user_id: String,
        is_premium_user: bool,
    ) -> Self {
        Self {
            access_token,
            refresh_token,
            user_id,
            is_premium_user,
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "SavedTokens(user_id='{}', is_premium={})",
            self.user_id, self.is_premium_user
        )
    }
}

impl From<anylist_rs::SavedTokens> for SavedTokens {
    fn from(t: anylist_rs::SavedTokens) -> Self {
        Self {
            access_token: t.access_token().to_string(),
            refresh_token: t.refresh_token().to_string(),
            user_id: t.user_id().to_string(),
            is_premium_user: t.is_premium_user(),
        }
    }
}

impl From<SavedTokens> for anylist_rs::SavedTokens {
    fn from(t: SavedTokens) -> Self {
        anylist_rs::SavedTokens::new(t.access_token, t.refresh_token, t.user_id, t.is_premium_user)
    }
}

/// A shopping list item
#[pyclass]
#[derive(Clone)]
pub struct ListItem {
    #[pyo3(get)]
    pub id: String,
    #[pyo3(get)]
    pub list_id: String,
    #[pyo3(get)]
    pub name: String,
    #[pyo3(get)]
    pub details: String,
    #[pyo3(get)]
    pub is_checked: bool,
    #[pyo3(get)]
    pub quantity: Option<String>,
    #[pyo3(get)]
    pub category: Option<String>,
    #[pyo3(get)]
    pub user_id: Option<String>,
    /// Product UPC/barcode if set
    #[pyo3(get)]
    pub product_upc: Option<String>,
}

#[pymethods]
impl ListItem {
    fn __repr__(&self) -> String {
        let checked = if self.is_checked { "✓" } else { "○" };
        format!("ListItem({} '{}' ({}))", checked, self.name, self.id)
    }
}

impl From<anylist_rs::ListItem> for ListItem {
    fn from(item: anylist_rs::ListItem) -> Self {
        Self {
            id: item.id().to_string(),
            list_id: item.list_id().to_string(),
            name: item.name().to_string(),
            details: item.details().to_string(),
            is_checked: item.is_checked(),
            quantity: item.quantity().map(|s| s.to_string()),
            category: item.category().map(|s| s.to_string()),
            user_id: item.user_id().map(|s| s.to_string()),
            product_upc: item.product_upc().map(|s| s.to_string()),
        }
    }
}

/// A shopping list
#[pyclass]
#[derive(Clone)]
pub struct ShoppingList {
    #[pyo3(get)]
    pub id: String,
    #[pyo3(get)]
    pub name: String,
    #[pyo3(get)]
    pub items: Vec<ListItem>,
}

#[pymethods]
impl ShoppingList {
    fn __repr__(&self) -> String {
        format!("ShoppingList('{}', {} items)", self.name, self.items.len())
    }

    fn __len__(&self) -> usize {
        self.items.len()
    }
}

impl From<anylist_rs::List> for ShoppingList {
    fn from(list: anylist_rs::List) -> Self {
        Self {
            id: list.id().to_string(),
            name: list.name().to_string(),
            items: list.items().iter().cloned().map(ListItem::from).collect(),
        }
    }
}

/// A favourite item
#[pyclass]
#[derive(Clone)]
pub struct FavouriteItem {
    #[pyo3(get)]
    pub id: String,
    #[pyo3(get)]
    pub list_id: String,
    #[pyo3(get)]
    pub name: String,
    #[pyo3(get)]
    pub quantity: Option<String>,
    #[pyo3(get)]
    pub details: Option<String>,
    #[pyo3(get)]
    pub category: Option<String>,
}

#[pymethods]
impl FavouriteItem {
    fn __repr__(&self) -> String {
        format!("FavouriteItem('{}' ({}))", self.name, self.id)
    }
}

impl From<anylist_rs::FavouriteItem> for FavouriteItem {
    fn from(item: anylist_rs::FavouriteItem) -> Self {
        Self {
            id: item.id().to_string(),
            list_id: item.list_id().to_string(),
            name: item.name().to_string(),
            quantity: item.quantity().map(|s| s.to_string()),
            details: item.details().map(|s| s.to_string()),
            category: item.category().map(|s| s.to_string()),
        }
    }
}

/// A favourites list
#[pyclass]
#[derive(Clone)]
pub struct FavouritesList {
    #[pyo3(get)]
    pub id: String,
    #[pyo3(get)]
    pub name: String,
    #[pyo3(get)]
    pub items: Vec<FavouriteItem>,
    #[pyo3(get)]
    pub shopping_list_id: Option<String>,
}

#[pymethods]
impl FavouritesList {
    fn __repr__(&self) -> String {
        format!(
            "FavouritesList('{}', {} items)",
            self.name,
            self.items.len()
        )
    }

    fn __len__(&self) -> usize {
        self.items.len()
    }
}

impl From<anylist_rs::FavouritesList> for FavouritesList {
    fn from(list: anylist_rs::FavouritesList) -> Self {
        Self {
            id: list.id().to_string(),
            name: list.name().to_string(),
            items: list.items().iter().cloned().map(FavouriteItem::from).collect(),
            shopping_list_id: list.shopping_list_id().map(|s| s.to_string()),
        }
    }
}

// ============================================================================
// Recipes
// ============================================================================

/// A recipe ingredient
#[pyclass]
#[derive(Clone)]
pub struct Ingredient {
    #[pyo3(get)]
    pub name: String,
    #[pyo3(get)]
    pub quantity: Option<String>,
    #[pyo3(get)]
    pub note: Option<String>,
    #[pyo3(get)]
    pub raw_ingredient: Option<String>,
}

#[pymethods]
impl Ingredient {
    #[new]
    #[pyo3(signature = (name, quantity=None, note=None, raw_ingredient=None))]
    fn new(
        name: String,
        quantity: Option<String>,
        note: Option<String>,
        raw_ingredient: Option<String>,
    ) -> Self {
        Self {
            name,
            quantity,
            note,
            raw_ingredient,
        }
    }

    fn __repr__(&self) -> String {
        if let Some(ref qty) = self.quantity {
            format!("Ingredient('{}', qty='{}')", self.name, qty)
        } else {
            format!("Ingredient('{}')", self.name)
        }
    }
}

impl From<anylist_rs::Ingredient> for Ingredient {
    fn from(i: anylist_rs::Ingredient) -> Self {
        Self {
            name: i.name().to_string(),
            quantity: i.quantity().map(|s| s.to_string()),
            note: i.note().map(|s| s.to_string()),
            raw_ingredient: i.raw_ingredient().map(|s| s.to_string()),
        }
    }
}

impl From<Ingredient> for anylist_rs::Ingredient {
    fn from(i: Ingredient) -> Self {
        let mut ingredient = anylist_rs::Ingredient::new(i.name);
        if let Some(qty) = i.quantity {
            ingredient = ingredient.quantity_of(qty);
        }
        if let Some(note) = i.note {
            ingredient = ingredient.note_of(note);
        }
        if let Some(raw) = i.raw_ingredient {
            ingredient = ingredient.raw_ingredient_of(raw);
        }
        ingredient
    }
}

/// A recipe
#[pyclass]
#[derive(Clone)]
pub struct Recipe {
    #[pyo3(get)]
    pub id: String,
    #[pyo3(get)]
    pub name: String,
    #[pyo3(get)]
    pub ingredients: Vec<Ingredient>,
    #[pyo3(get)]
    pub preparation_steps: Vec<String>,
    #[pyo3(get)]
    pub note: Option<String>,
    #[pyo3(get)]
    pub source_name: Option<String>,
    #[pyo3(get)]
    pub source_url: Option<String>,
    #[pyo3(get)]
    pub servings: Option<String>,
    #[pyo3(get)]
    pub prep_time: Option<i32>,
    #[pyo3(get)]
    pub cook_time: Option<i32>,
    #[pyo3(get)]
    pub rating: Option<i32>,
    #[pyo3(get)]
    pub photo_urls: Vec<String>,
}

#[pymethods]
impl Recipe {
    fn __repr__(&self) -> String {
        format!(
            "Recipe('{}', {} ingredients, {} steps)",
            self.name,
            self.ingredients.len(),
            self.preparation_steps.len()
        )
    }
}

impl From<anylist_rs::Recipe> for Recipe {
    fn from(r: anylist_rs::Recipe) -> Self {
        Self {
            id: r.id().to_string(),
            name: r.name().to_string(),
            ingredients: r
                .ingredients()
                .iter()
                .cloned()
                .map(Ingredient::from)
                .collect(),
            preparation_steps: r.preparation_steps().to_vec(),
            note: r.note().map(|s| s.to_string()),
            source_name: r.source_name().map(|s| s.to_string()),
            source_url: r.source_url().map(|s| s.to_string()),
            servings: r.servings().map(|s| s.to_string()),
            prep_time: r.prep_time(),
            cook_time: r.cook_time(),
            rating: r.rating(),
            photo_urls: r.photo_urls().to_vec(),
        }
    }
}

// ============================================================================
// iCalendar
// ============================================================================

/// iCalendar information for meal planning calendar
#[pyclass]
#[derive(Clone)]
pub struct ICalendarInfo {
    /// Whether iCalendar export is enabled
    #[pyo3(get)]
    pub enabled: bool,
    /// The iCalendar URL (if enabled)
    #[pyo3(get)]
    pub url: Option<String>,
    /// The token/ID used in the URL
    #[pyo3(get)]
    pub token: Option<String>,
}

#[pymethods]
impl ICalendarInfo {
    fn __repr__(&self) -> String {
        if let Some(ref url) = self.url {
            format!("ICalendarInfo(enabled={}, url='{}')", self.enabled, url)
        } else {
            format!("ICalendarInfo(enabled={})", self.enabled)
        }
    }
}

impl From<anylist_rs::ICalendarInfo> for ICalendarInfo {
    fn from(info: anylist_rs::ICalendarInfo) -> Self {
        Self {
            enabled: info.enabled(),
            url: info.url().map(|s| s.to_string()),
            token: info.token().map(|s| s.to_string()),
        }
    }
}

// ============================================================================
// Real-time Sync
// ============================================================================

/// Sync event types from real-time WebSocket connection
#[pyclass(eq, eq_int)]
#[derive(Clone, PartialEq, Eq)]
pub enum SyncEvent {
    /// Shopping lists have changed
    ShoppingListsChanged,
    /// Categorized items have changed
    CategorizedItemsChanged,
    /// List folders have changed
    ListFoldersChanged,
    /// List settings have changed
    ListSettingsChanged,
    /// Starter lists (favourites) have changed
    StarterListsChanged,
    /// Starter list order has changed
    StarterListOrderChanged,
    /// Starter list settings have changed
    StarterListSettingsChanged,
    /// Mobile app settings have changed
    MobileAppSettingsChanged,
    /// User categories have changed
    UserCategoriesChanged,
    /// Recipe data has changed
    RecipeDataChanged,
    /// Meal plan calendar has changed
    MealPlanCalendarChanged,
    /// Account info has changed
    AccountInfoChanged,
    /// Subscription info has changed
    SubscriptionInfoChanged,
    /// Account has been deleted
    AccountDeleted,
}

impl From<anylist_rs::SyncEvent> for SyncEvent {
    fn from(event: anylist_rs::SyncEvent) -> Self {
        match event {
            anylist_rs::SyncEvent::ShoppingListsChanged => SyncEvent::ShoppingListsChanged,
            anylist_rs::SyncEvent::CategorizedItemsChanged => SyncEvent::CategorizedItemsChanged,
            anylist_rs::SyncEvent::ListFoldersChanged => SyncEvent::ListFoldersChanged,
            anylist_rs::SyncEvent::ListSettingsChanged => SyncEvent::ListSettingsChanged,
            anylist_rs::SyncEvent::StarterListsChanged => SyncEvent::StarterListsChanged,
            anylist_rs::SyncEvent::StarterListOrderChanged => SyncEvent::StarterListOrderChanged,
            anylist_rs::SyncEvent::StarterListSettingsChanged => {
                SyncEvent::StarterListSettingsChanged
            }
            anylist_rs::SyncEvent::MobileAppSettingsChanged => SyncEvent::MobileAppSettingsChanged,
            anylist_rs::SyncEvent::UserCategoriesChanged => SyncEvent::UserCategoriesChanged,
            anylist_rs::SyncEvent::RecipeDataChanged => SyncEvent::RecipeDataChanged,
            anylist_rs::SyncEvent::MealPlanCalendarChanged => SyncEvent::MealPlanCalendarChanged,
            anylist_rs::SyncEvent::AccountInfoChanged => SyncEvent::AccountInfoChanged,
            anylist_rs::SyncEvent::SubscriptionInfoChanged => SyncEvent::SubscriptionInfoChanged,
            anylist_rs::SyncEvent::AccountDeleted => SyncEvent::AccountDeleted,
            anylist_rs::SyncEvent::Heartbeat => SyncEvent::ShoppingListsChanged, // Should never happen, filtered out
        }
    }
}

/// Real-time sync connection state
#[pyclass(eq, eq_int)]
#[derive(Clone, PartialEq, Eq)]
pub enum ConnectionState {
    /// Not connected
    Disconnected,
    /// Connecting to server
    Connecting,
    /// Connected and active
    Connected,
    /// Connection lost, will retry
    Reconnecting,
    /// Permanently closed
    Closed,
}

impl From<anylist_rs::realtime::ConnectionState> for ConnectionState {
    fn from(state: anylist_rs::realtime::ConnectionState) -> Self {
        match state {
            anylist_rs::realtime::ConnectionState::Disconnected => ConnectionState::Disconnected,
            anylist_rs::realtime::ConnectionState::Connecting => ConnectionState::Connecting,
            anylist_rs::realtime::ConnectionState::Connected => ConnectionState::Connected,
            anylist_rs::realtime::ConnectionState::Reconnecting => ConnectionState::Reconnecting,
            anylist_rs::realtime::ConnectionState::Closed => ConnectionState::Closed,
        }
    }
}

/// Real-time sync manager for receiving live updates
///
/// Use `poll_events()` to retrieve events that have been received since the last poll.
#[pyclass]
pub struct RealtimeSync {
    runtime: Arc<tokio::runtime::Runtime>,
    sync: Arc<Mutex<Option<anylist_rs::realtime::RealtimeSync>>>,
    event_queue: Arc<std::sync::Mutex<VecDeque<SyncEvent>>>,
}

#[pymethods]
impl RealtimeSync {
    /// Get the current connection state
    fn state(&self) -> PyResult<ConnectionState> {
        let sync_arc = self.sync.clone();

        self.runtime.block_on(async {
            let sync = sync_arc.lock().await;
            let sync_ref = sync
                .as_ref()
                .ok_or_else(|| PyRuntimeError::new_err("Sync not initialized"))?;

            Ok(ConnectionState::from(sync_ref.state().await))
        })
    }

    /// Check if currently connected
    fn is_connected(&self) -> PyResult<bool> {
        let sync_arc = self.sync.clone();

        self.runtime.block_on(async {
            let sync = sync_arc.lock().await;
            let sync_ref = sync
                .as_ref()
                .ok_or_else(|| PyRuntimeError::new_err("Sync not initialized"))?;

            Ok(sync_ref.is_connected().await)
        })
    }

    /// Poll for new events
    ///
    /// Returns a list of events that have been received since the last poll.
    /// Events are cleared from the internal queue after being returned.
    fn poll_events(&self) -> PyResult<Vec<SyncEvent>> {
        let mut queue = self
            .event_queue
            .lock()
            .map_err(|e| PyRuntimeError::new_err(format!("Lock error: {}", e)))?;

        Ok(queue.drain(..).collect())
    }

    /// Disconnect and stop receiving events
    fn disconnect(&self) -> PyResult<()> {
        let sync_arc = self.sync.clone();

        self.runtime.block_on(async {
            let mut sync = sync_arc.lock().await;
            if let Some(ref mut s) = *sync {
                s.disconnect()
                    .await
                    .map_err(|e| PyRuntimeError::new_err(format!("Disconnect failed: {}", e)))?;
            }
            *sync = None;
            Ok(())
        })
    }
}

/// The main AnyList client for Python
#[pyclass]
pub struct AnyListClient {
    // Store as Arc for realtime sync compatibility
    client: Arc<anylist_rs::AnyListClient>,
    runtime: Arc<tokio::runtime::Runtime>,
}

#[pymethods]
impl AnyListClient {
    /// Create a new client by logging in with email and password
    #[staticmethod]
    fn login(_py: Python<'_>, email: String, password: String) -> PyResult<Self> {
        let runtime =
            Arc::new(tokio::runtime::Runtime::new().map_err(|e| {
                PyRuntimeError::new_err(format!("Failed to create runtime: {}", e))
            })?);

        let client = runtime
            .block_on(async { anylist_rs::AnyListClient::login(&email, &password).await })
            .map_err(|e| PyRuntimeError::new_err(format!("Login failed: {}", e)))?;

        Ok(Self {
            client: Arc::new(client),
            runtime,
        })
    }

    /// Create a client from previously saved tokens
    #[staticmethod]
    fn from_tokens(tokens: SavedTokens) -> PyResult<Self> {
        let runtime =
            Arc::new(tokio::runtime::Runtime::new().map_err(|e| {
                PyRuntimeError::new_err(format!("Failed to create runtime: {}", e))
            })?);

        let rust_tokens: anylist_rs::SavedTokens = tokens.into();
        let client = anylist_rs::AnyListClient::from_tokens(rust_tokens)
            .map_err(|e| PyRuntimeError::new_err(format!("Failed to restore session: {}", e)))?;

        Ok(Self {
            client: Arc::new(client),
            runtime,
        })
    }

    /// Export tokens for persistent storage
    fn export_tokens(&self) -> PyResult<SavedTokens> {
        let tokens = self
            .client
            .export_tokens()
            .map_err(|e| PyRuntimeError::new_err(format!("Failed to export tokens: {}", e)))?;

        Ok(SavedTokens::from(tokens))
    }

    /// Get the user ID
    fn user_id(&self) -> String {
        self.client.user_id()
    }

    /// Check if user has premium subscription
    fn is_premium_user(&self) -> bool {
        self.client.is_premium_user()
    }

    // ========================================================================
    // List Operations
    // ========================================================================

    /// Get all shopping lists
    fn get_lists(&self) -> PyResult<Vec<ShoppingList>> {
        let client = self.client.clone();
        self.runtime.block_on(async {
            let lists = client
                .get_lists()
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Failed to get lists: {}", e)))?;
            Ok(lists.into_iter().map(ShoppingList::from).collect())
        })
    }

    /// Get a list by ID
    fn get_list_by_id(&self, list_id: String) -> PyResult<ShoppingList> {
        let client = self.client.clone();
        self.runtime.block_on(async {
            let list = client
                .get_list_by_id(&list_id)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Failed to get list: {}", e)))?;
            Ok(ShoppingList::from(list))
        })
    }

    /// Get a list by name
    fn get_list_by_name(&self, name: String) -> PyResult<ShoppingList> {
        let client = self.client.clone();
        self.runtime.block_on(async {
            let list = client
                .get_list_by_name(&name)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Failed to get list: {}", e)))?;
            Ok(ShoppingList::from(list))
        })
    }

    /// Create a new shopping list
    fn create_list(&self, name: String) -> PyResult<ShoppingList> {
        let client = self.client.clone();
        self.runtime.block_on(async {
            let list = client
                .create_list(&name)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Failed to create list: {}", e)))?;
            Ok(ShoppingList::from(list))
        })
    }

    /// Delete a shopping list
    fn delete_list(&self, list_id: String) -> PyResult<()> {
        let client = self.client.clone();
        self.runtime.block_on(async {
            client
                .delete_list(&list_id)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Failed to delete list: {}", e)))
        })
    }

    /// Rename a shopping list
    fn rename_list(&self, list_id: String, new_name: String) -> PyResult<()> {
        let client = self.client.clone();
        self.runtime.block_on(async {
            client
                .rename_list(&list_id, &new_name)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Failed to rename list: {}", e)))
        })
    }

    // ========================================================================
    // Item Operations
    // ========================================================================

    /// Add an item to a shopping list
    fn add_item(&self, list_id: String, name: String) -> PyResult<ListItem> {
        let client = self.client.clone();
        self.runtime.block_on(async {
            let item = client
                .add_item(&list_id, &name)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Failed to add item: {}", e)))?;
            Ok(ListItem::from(item))
        })
    }

    /// Add an item with details
    #[pyo3(signature = (list_id, name, quantity=None, details=None, category=None))]
    fn add_item_with_details(
        &self,
        list_id: String,
        name: String,
        quantity: Option<String>,
        details: Option<String>,
        category: Option<String>,
    ) -> PyResult<ListItem> {
        let client = self.client.clone();
        self.runtime.block_on(async {
            let item = client
                .add_item_with_details(
                    &list_id,
                    &name,
                    quantity.as_deref(),
                    details.as_deref(),
                    category.as_deref(),
                )
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Failed to add item: {}", e)))?;
            Ok(ListItem::from(item))
        })
    }

    /// Delete an item from a list
    fn delete_item(&self, list_id: String, item_id: String) -> PyResult<()> {
        let client = self.client.clone();
        self.runtime.block_on(async {
            client
                .delete_item(&list_id, &item_id)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Failed to delete item: {}", e)))
        })
    }

    /// Cross off (check) an item
    fn cross_off_item(&self, list_id: String, item_id: String) -> PyResult<()> {
        let client = self.client.clone();
        self.runtime.block_on(async {
            client
                .cross_off_item(&list_id, &item_id)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Failed to cross off item: {}", e)))
        })
    }

    /// Uncheck an item
    fn uncheck_item(&self, list_id: String, item_id: String) -> PyResult<()> {
        let client = self.client.clone();
        self.runtime.block_on(async {
            client
                .uncheck_item(&list_id, &item_id)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Failed to uncheck item: {}", e)))
        })
    }

    /// Delete all crossed-off items from a list
    fn delete_all_crossed_off_items(&self, list_id: String) -> PyResult<()> {
        let client = self.client.clone();
        self.runtime.block_on(async {
            client
                .delete_all_crossed_off_items(&list_id)
                .await
                .map_err(|e| {
                    PyRuntimeError::new_err(format!("Failed to delete crossed off items: {}", e))
                })
        })
    }

    // ========================================================================
    // Favourites Operations
    // ========================================================================

    /// Get all favourite items
    fn get_favourites(&self) -> PyResult<Vec<FavouriteItem>> {
        let client = self.client.clone();
        self.runtime.block_on(async {
            let favourites = client
                .get_favourites()
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Failed to get favourites: {}", e)))?;
            Ok(favourites.into_iter().map(FavouriteItem::from).collect())
        })
    }

    /// Get all favourites lists
    fn get_favourites_lists(&self) -> PyResult<Vec<FavouritesList>> {
        let client = self.client.clone();
        self.runtime.block_on(async {
            let lists = client.get_favourites_lists().await.map_err(|e| {
                PyRuntimeError::new_err(format!("Failed to get favourites lists: {}", e))
            })?;
            Ok(lists.into_iter().map(FavouritesList::from).collect())
        })
    }

    /// Add a favourite item
    #[pyo3(signature = (name, category=None))]
    fn add_favourite(&self, name: String, category: Option<String>) -> PyResult<FavouriteItem> {
        let client = self.client.clone();
        self.runtime.block_on(async {
            let item = client
                .add_favourite(&name, category.as_deref())
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Failed to add favourite: {}", e)))?;
            Ok(FavouriteItem::from(item))
        })
    }

    /// Remove a favourite item
    fn remove_favourite(&self, list_id: String, item_id: String) -> PyResult<()> {
        let client = self.client.clone();
        self.runtime.block_on(async {
            client
                .remove_favourite(&list_id, &item_id)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Failed to remove favourite: {}", e)))
        })
    }

    // ========================================================================
    // Recipes
    // ========================================================================

    /// Get all recipes
    fn get_recipes(&self) -> PyResult<Vec<Recipe>> {
        let client = self.client.clone();
        self.runtime.block_on(async {
            let recipes = client
                .get_recipes()
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Failed to get recipes: {}", e)))?;
            Ok(recipes.into_iter().map(Recipe::from).collect())
        })
    }

    /// Get a recipe by ID
    fn get_recipe_by_id(&self, recipe_id: String) -> PyResult<Recipe> {
        let client = self.client.clone();
        self.runtime.block_on(async {
            let recipe = client
                .get_recipe_by_id(&recipe_id)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Failed to get recipe: {}", e)))?;
            Ok(Recipe::from(recipe))
        })
    }

    /// Get a recipe by name
    fn get_recipe_by_name(&self, name: String) -> PyResult<Recipe> {
        let client = self.client.clone();
        self.runtime.block_on(async {
            let recipe = client
                .get_recipe_by_name(&name)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Failed to get recipe: {}", e)))?;
            Ok(Recipe::from(recipe))
        })
    }

    /// Create a new recipe
    fn create_recipe(
        &self,
        name: String,
        ingredients: Vec<Ingredient>,
        preparation_steps: Vec<String>,
    ) -> PyResult<Recipe> {
        let client = self.client.clone();
        let rust_ingredients: Vec<anylist_rs::Ingredient> =
            ingredients.into_iter().map(|i| i.into()).collect();

        self.runtime.block_on(async {
            let recipe = client
                .create_recipe(&name, rust_ingredients, preparation_steps)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Failed to create recipe: {}", e)))?;
            Ok(Recipe::from(recipe))
        })
    }

    /// Update an existing recipe
    fn update_recipe(
        &self,
        recipe_id: String,
        name: String,
        ingredients: Vec<Ingredient>,
        preparation_steps: Vec<String>,
    ) -> PyResult<()> {
        let client = self.client.clone();
        let rust_ingredients: Vec<anylist_rs::Ingredient> =
            ingredients.into_iter().map(|i| i.into()).collect();

        self.runtime.block_on(async {
            client
                .update_recipe(&recipe_id, &name, rust_ingredients, preparation_steps)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Failed to update recipe: {}", e)))
        })
    }

    /// Delete a recipe
    fn delete_recipe(&self, recipe_id: String) -> PyResult<()> {
        let client = self.client.clone();
        self.runtime.block_on(async {
            client
                .delete_recipe(&recipe_id)
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Failed to delete recipe: {}", e)))
        })
    }

    /// Add recipe ingredients to a shopping list
    #[pyo3(signature = (recipe_id, list_id, scale_factor=None))]
    fn add_recipe_to_list(
        &self,
        recipe_id: String,
        list_id: String,
        scale_factor: Option<f64>,
    ) -> PyResult<()> {
        let client = self.client.clone();
        self.runtime.block_on(async {
            client
                .add_recipe_to_list(&recipe_id, &list_id, scale_factor)
                .await
                .map_err(|e| {
                    PyRuntimeError::new_err(format!("Failed to add recipe to list: {}", e))
                })
        })
    }

    // ========================================================================
    // iCalendar
    // ========================================================================

    /// Enable iCalendar export for meal planning calendar
    ///
    /// Returns ICalendarInfo with the URL that can be subscribed to by external
    /// calendar applications like Home Assistant, Google Calendar, etc.
    fn enable_icalendar(&self) -> PyResult<ICalendarInfo> {
        let client = self.client.clone();
        self.runtime.block_on(async {
            let info = client.enable_icalendar().await.map_err(|e| {
                PyRuntimeError::new_err(format!("Failed to enable iCalendar: {}", e))
            })?;
            Ok(ICalendarInfo::from(info))
        })
    }

    /// Disable iCalendar export for meal planning calendar
    fn disable_icalendar(&self) -> PyResult<()> {
        let client = self.client.clone();
        self.runtime.block_on(async {
            client
                .disable_icalendar()
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Failed to disable iCalendar: {}", e)))
        })
    }

    /// Get the iCalendar URL if already enabled
    fn get_icalendar_url(&self) -> PyResult<Option<String>> {
        let client = self.client.clone();
        self.runtime.block_on(async {
            client
                .get_icalendar_url()
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Failed to get iCalendar URL: {}", e)))
        })
    }

    // ========================================================================
    // Real-time Sync
    // ========================================================================

    /// Start real-time sync to receive live updates
    ///
    /// Returns a RealtimeSync object. Use `poll_events()` to get new events.
    ///
    /// Example:
    /// ```python
    /// sync = client.start_realtime_sync()
    /// while sync.is_connected():
    ///     events = sync.poll_events()
    ///     for event in events:
    ///         if event == SyncEvent.ShoppingListsChanged:
    ///             lists = client.get_lists()
    ///     time.sleep(1)
    /// sync.disconnect()
    /// ```
    fn start_realtime_sync(&self) -> PyResult<RealtimeSync> {
        let client_arc = self.client.clone();
        let runtime = self.runtime.clone();

        // Create event queue for collecting events
        let event_queue: Arc<std::sync::Mutex<VecDeque<SyncEvent>>> =
            Arc::new(std::sync::Mutex::new(VecDeque::new()));
        let event_queue_clone = event_queue.clone();

        let sync = self.runtime.block_on(async {
            let sync = client_arc
                .start_realtime_sync(move |event| {
                    // Filter out heartbeats
                    if event != anylist_rs::SyncEvent::Heartbeat {
                        if let Ok(mut queue) = event_queue_clone.lock() {
                            queue.push_back(SyncEvent::from(event));
                        }
                    }
                })
                .await
                .map_err(|e| PyRuntimeError::new_err(format!("Failed to start sync: {}", e)))?;

            Ok::<_, PyErr>(sync)
        })?;

        Ok(RealtimeSync {
            runtime,
            sync: Arc::new(Mutex::new(Some(sync))),
            event_queue,
        })
    }
}

/// The pyanylist Python module
#[pymodule]
fn pyanylist(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<AnyListClient>()?;
    m.add_class::<SavedTokens>()?;
    m.add_class::<ShoppingList>()?;
    m.add_class::<ListItem>()?;
    m.add_class::<FavouriteItem>()?;
    m.add_class::<FavouritesList>()?;
    m.add_class::<Recipe>()?;
    m.add_class::<Ingredient>()?;
    m.add_class::<ICalendarInfo>()?;
    m.add_class::<RealtimeSync>()?;
    m.add_class::<SyncEvent>()?;
    m.add_class::<ConnectionState>()?;
    Ok(())
}
