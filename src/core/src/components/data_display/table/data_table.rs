use icondata::{BsFilter, BsPencil, BsSortDown, BsSortUp, BsThreeDots, BsTrash, ImDrawer2};
use std::collections::HashMap;
use std::hash::Hash;
use std::hash::Hasher;

use chrono::DateTime;
use chrono::Utc;

use leptos::html::*;
use leptos::prelude::*;
use leptos_icons::Icon;

use super::pagination::Pagination;
use crate::components::actions::button::BasicButton;
use crate::components::feedback::popover::Popover;
use crate::utils::time::get_elapsed_time;

#[derive(Clone)]
pub struct Column {
    pub name: String,
    pub sortable: bool,
    pub sort_order: SortOrder,
    pub sort_icon: ViewFn,
}

impl std::fmt::Debug for Column {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Column")
            .field("name", &self.name)
            .field("sortable", &self.sortable)
            .field("sort_order", &self.sort_order)
            .field("sort_icon", &"<ViewFn>")
            .finish()
    }
}

#[derive(Clone, PartialEq, Debug, Default)]
pub enum SortOrder {
    #[default]
    Default,
    Ascending,
    Descending,
}

/// Represents a table column with an optional sort capability.
///
/// Use `Column::new` to construct, then `toggle_sort` and `toggle_sort_icon`
/// to cycle through `Default → Ascending → Descending → Default`.
impl Column {
    pub fn new(name: &str, sortable: bool) -> Self {
        Column {
            name: name.to_string(),
            sortable,
            sort_order: Default::default(),
            sort_icon: (|| view! { <Icon width="0.8em" height="0.8em" icon=BsFilter /> }).into(),
        }
    }

    /// This method toggles the sort order of the column.
    pub fn toggle_sort(&mut self) -> &mut Self {
        self.sort_order = match self.sort_order {
            SortOrder::Default => SortOrder::Ascending,
            SortOrder::Ascending => SortOrder::Descending,
            SortOrder::Descending => SortOrder::Default,
        };
        self
    }

    /// This method toggles the sort icon of the column.
    pub fn toggle_sort_icon(&mut self) -> &mut Self {
        self.sort_icon = match self.sort_order {
            SortOrder::Default => {
                (|| view! { <Icon width="0.8em" height="0.8em" icon=BsFilter /> }).into()
            }
            SortOrder::Ascending => {
                (|| view! { <Icon width="0.8em" height="0.8em" icon=BsSortUp /> }).into()
            }
            SortOrder::Descending => {
                (|| view! { <Icon width="0.8em" height="0.8em" icon=BsSortDown /> }).into()
            }
        };
        self
    }
}

impl PartialEq for Column {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
        // sort_icon (ViewFn) is not compared
    }
}

/// Represents the value of a single table cell.
///
/// - `String(String)` – Plain text.
/// - `Int32(i32)`, `Int64(i64)` – Signed integers.
/// - `UInt32(u32)`, `UInt64(u64)`, `UInt128(u128)`, `Usize(usize)` – Unsigned integers.
/// - `Float32(f32)`, `Float64(f64)` – Floating point, rendered to 2 decimal places.
/// - `Bool(bool)` – Rendered as `"true"` or `"false"`.
/// - `Html(ViewFn)` – Arbitrary HTML content rendered inline.
/// - `DateTime(String)` – RFC3339 string rendered as `dd Mon YYYY`.
/// - `Duration(String)` – RFC3339 start date; rendered as elapsed time from now.
#[derive(Clone)]
#[allow(dead_code)]
pub enum TableCellData {
    /// This handles String data type
    String(String),
    /// This handles i32 data type
    Int32(i32),
    /// This handles i64 data type
    Int64(i64),
    /// This handles RSX which is a ViewFn that returns HTML content
    Html(ViewFn), // Simplified for Leptos; assumes HTML as ViewFn
    /// This handles f32 data type
    Float32(f32),
    /// This handles f64 data type
    Float64(f64),
    /// This handles usize data type
    Usize(usize),
    /// This handles u32 data type
    UInt32(u32),
    /// This handles u64 data type
    UInt64(u64),
    /// This handles u128 data type
    UInt128(u128),
    /// This handles bool data type
    Bool(bool),
    /// This handles DateTime data and takes an RFC3339 formatted string
    DateTime(String),
    /// This handles time duration and takes an RFC3339 formatted string which is the start date. End date is assumed to be the current date.
    Duration(String),
}

impl std::fmt::Debug for TableCellData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TableCellData::String(s) => f.debug_tuple("String").field(s).finish(),
            TableCellData::Int32(i) => f.debug_tuple("Int32").field(i).finish(),
            TableCellData::Int64(i) => f.debug_tuple("Int64").field(i).finish(),
            TableCellData::Html(_) => f.debug_tuple("Html").field(&"<Html content>").finish(),
            TableCellData::Float32(f32) => f.debug_tuple("Float32").field(f32).finish(),
            TableCellData::Float64(f64) => f.debug_tuple("Float64").field(f64).finish(),
            TableCellData::Usize(u) => f.debug_tuple("Usize").field(u).finish(),
            TableCellData::UInt32(u) => f.debug_tuple("UInt32").field(u).finish(),
            TableCellData::UInt64(u) => f.debug_tuple("UInt64").field(u).finish(),
            TableCellData::UInt128(u) => f.debug_tuple("UInt128").field(u).finish(),
            TableCellData::Bool(b) => f.debug_tuple("Bool").field(b).finish(),
            TableCellData::DateTime(dt) => f.debug_tuple("DateTime").field(dt).finish(),
            TableCellData::Duration(d) => f.debug_tuple("Duration").field(d).finish(),
        }
    }
}

impl PartialEq for TableCellData {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (TableCellData::String(a), TableCellData::String(b)) => a == b,
            (TableCellData::Int32(a), TableCellData::Int32(b)) => a == b,
            (TableCellData::Int64(a), TableCellData::Int64(b)) => a == b,
            (TableCellData::Float32(a), TableCellData::Float32(b)) => a == b,
            (TableCellData::Float64(a), TableCellData::Float64(b)) => a == b,
            (TableCellData::UInt32(a), TableCellData::UInt32(b)) => a == b,
            (TableCellData::UInt64(a), TableCellData::UInt64(b)) => a == b,
            (TableCellData::UInt128(a), TableCellData::UInt128(b)) => a == b,
            (TableCellData::Bool(a), TableCellData::Bool(b)) => a == b,
            (TableCellData::DateTime(a), TableCellData::DateTime(b)) => {
                match DateTime::parse_from_rfc3339(a) {
                    Ok(a) => match DateTime::parse_from_rfc3339(b) {
                        Ok(b) => a.timestamp() == b.timestamp(),
                        Err(_) => false,
                    },
                    Err(_) => false,
                }
            }
            (TableCellData::Duration(a), TableCellData::Duration(b)) => {
                match DateTime::parse_from_rfc3339(a) {
                    Ok(a) => match DateTime::parse_from_rfc3339(b) {
                        Ok(b) => a.timestamp() == b.timestamp(),
                        Err(_) => false,
                    },
                    Err(_) => false,
                }
            }
            (TableCellData::Html(_), TableCellData::Html(_)) => false, // ViewFn cannot be compared
            _ => false, // Different variants are not equal
        }
    }
}

impl Eq for TableCellData {}

impl Hash for TableCellData {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            TableCellData::String(s) | TableCellData::DateTime(s) | TableCellData::Duration(s) => {
                0u8.hash(state); // Variant discriminator
                s.hash(state);
            }
            TableCellData::Int32(i) => {
                1u8.hash(state);
                i.hash(state);
            }
            TableCellData::Int64(i) => {
                2u8.hash(state);
                i.hash(state);
            }
            TableCellData::Float32(f) => {
                3u8.hash(state);
                // Convert float to bits to handle NaN and other edge cases
                f.to_bits().hash(state);
            }
            TableCellData::Float64(f) => {
                4u8.hash(state);
                // Convert float to bits to handle NaN and other edge cases
                f.to_bits().hash(state);
            }
            TableCellData::UInt32(u) => {
                5u8.hash(state);
                u.hash(state);
            }
            TableCellData::UInt64(u) => {
                6u8.hash(state);
                u.hash(state);
            }
            TableCellData::UInt128(u) => {
                7u8.hash(state);
                u.hash(state);
            }
            _ => {
                8u8.hash(state);
                // Since ViewFn can't be hashed, use a constant or skip
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct TableProps {
    pub columns: Vec<Column>,
    pub data: Vec<HashMap<String, TableCellData>>,
    pub page_size: usize,
    pub on_row_click: Callback<HashMap<String, TableCellData>>,
    pub on_row_action: Callback<(HashMap<String, TableCellData>, String)>,
    pub editable: bool,
    pub deletable: bool,
}

// Manually implement PartialEq, ignoring Callback fields
impl PartialEq for TableProps {
    fn eq(&self, other: &Self) -> bool {
        self.columns == other.columns
            && self.data == other.data
            && self.page_size == other.page_size
            && self.editable == other.editable
            && self.deletable == other.deletable
        // Note: Callbacks are not compared
    }
}

impl TableProps {
    /// This method paginates the table data based on the current page and page size.
    pub fn paginate(
        &mut self,
        current_page: usize,
    ) -> (usize, usize, Vec<HashMap<String, TableCellData>>) {
        let total_pages = (self.data.len() as f64 / self.page_size as f64).ceil() as usize;
        let current_data = self
            .data
            .iter()
            .skip((current_page - 1) * self.page_size)
            .take(self.page_size)
            .map(|row| row.clone())
            .collect();
        (current_page, total_pages, current_data)
    }

    /// This method sorts the table data based on the specified column. This is triggered when the user clicks on a column header.
    pub fn sort(&mut self, column: &Column) -> Vec<HashMap<String, TableCellData>> {
        match column.sort_order {
            SortOrder::Ascending => {
                self.data
                    .sort_by(|a, b| match (a.get(&column.name), b.get(&column.name)) {
                        (Some(TableCellData::String(a)), Some(TableCellData::String(b))) => {
                            a.cmp(b)
                        }
                        (Some(TableCellData::Int32(a)), Some(TableCellData::Int32(b))) => a.cmp(b),
                        (Some(TableCellData::Int64(a)), Some(TableCellData::Int64(b))) => a.cmp(b),
                        (Some(TableCellData::Float32(a)), Some(TableCellData::Float32(b))) => {
                            a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)
                        }
                        (Some(TableCellData::Float64(a)), Some(TableCellData::Float64(b))) => {
                            a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)
                        }
                        (Some(TableCellData::UInt32(a)), Some(TableCellData::UInt32(b))) => {
                            a.cmp(b)
                        }
                        (Some(TableCellData::UInt64(a)), Some(TableCellData::UInt64(b))) => {
                            a.cmp(b)
                        }
                        (Some(TableCellData::UInt128(a)), Some(TableCellData::UInt128(b))) => {
                            a.cmp(b)
                        }
                        (Some(TableCellData::Bool(a)), Some(TableCellData::Bool(b))) => a.cmp(b),
                        (Some(TableCellData::DateTime(a)), Some(TableCellData::DateTime(b))) => {
                            a.cmp(b)
                        }
                        (Some(TableCellData::Duration(a)), Some(TableCellData::Duration(b))) => {
                            a.cmp(b)
                        }
                        _ => std::cmp::Ordering::Equal,
                    });
            }
            SortOrder::Descending => {
                self.data
                    .sort_by(|a, b| match (a.get(&column.name), b.get(&column.name)) {
                        (Some(TableCellData::String(a)), Some(TableCellData::String(b))) => {
                            b.cmp(a)
                        }
                        (Some(TableCellData::Int32(a)), Some(TableCellData::Int32(b))) => b.cmp(a),
                        (Some(TableCellData::Int64(a)), Some(TableCellData::Int64(b))) => b.cmp(a),
                        (Some(TableCellData::UInt32(a)), Some(TableCellData::UInt32(b))) => {
                            b.cmp(a)
                        }
                        (Some(TableCellData::UInt64(a)), Some(TableCellData::UInt64(b))) => {
                            b.cmp(a)
                        }
                        (Some(TableCellData::UInt128(a)), Some(TableCellData::UInt128(b))) => {
                            b.cmp(a)
                        }
                        (Some(TableCellData::Bool(a)), Some(TableCellData::Bool(b))) => b.cmp(a),
                        (Some(TableCellData::Float32(a)), Some(TableCellData::Float32(b))) => {
                            b.partial_cmp(a).unwrap_or(std::cmp::Ordering::Equal)
                        }
                        (Some(TableCellData::Float64(a)), Some(TableCellData::Float64(b))) => {
                            b.partial_cmp(a).unwrap_or(std::cmp::Ordering::Equal)
                        }
                        (Some(TableCellData::DateTime(a)), Some(TableCellData::DateTime(b))) => {
                            b.cmp(a)
                        }
                        (Some(TableCellData::Duration(a)), Some(TableCellData::Duration(b))) => {
                            b.cmp(a)
                        }
                        _ => std::cmp::Ordering::Equal,
                    });
            }
            SortOrder::Default => {}
        }
        self.data.to_owned()
    }
}

/// A sortable, paginated data table with optional row-level edit and delete actions.
///
/// Each row is represented as a `HashMap<String, TableCellData>` where keys match column names.
/// Every row **must** include an `"id"` key with a `TableCellData::String` value for keying to work correctly.
///
/// # Props
///
/// - `data` – `RwSignal<(Vec<Column>, Vec<HashMap<String, TableCellData>>)>` holding columns and rows.
/// - `page_size` – Number of rows per page. Defaults to `10`.
/// - `on_row_click` – Callback fired when a row is clicked, receives the row's data map.
/// - `on_row_action` – Callback fired when an action button is clicked, receives `(row_data, action_type)` where `action_type` is `"edit"` or `"delete"`.
/// - `editable` – Shows an edit action button per row. Defaults to `false`.
/// - `deletable` – Shows a delete action button per row. Defaults to `false`.
///
/// # Example
///
/// ```
/// use leptos::prelude::*;
/// use std::collections::HashMap;
/// use detaxine_ui::components::data_display::table::data_table::{Column, TableCellData, DataTable};
///
/// #[component]
/// fn Example() -> impl IntoView {
///     let columns = vec![
///         Column::new("Name", true),
///         Column::new("Age", true),
///     ];
///     let rows = vec![{
///         let mut row = HashMap::new();
///         row.insert("id".to_string(), TableCellData::String("1".to_string()));
///         row.insert("Name".to_string(), TableCellData::String("Alice".to_string()));
///         row.insert("Age".to_string(), TableCellData::UInt32(30));
///         row
///     }];
///     let data = RwSignal::new((columns, rows));
///     view! {
///         <DataTable data=data editable=true deletable=true />
///     }
/// }
/// ```
#[component]
pub fn DataTable(
    #[prop(into)] data: RwSignal<(Vec<Column>, Vec<HashMap<String, TableCellData>>)>,
    #[prop(optional, default = 10)] page_size: usize,
    #[prop(optional, default = Callback::new(|_| {}))] on_row_click: Callback<
        HashMap<String, TableCellData>,
    >,
    #[prop(optional, default = Callback::new(|_| {}))] on_row_action: Callback<(
        HashMap<String, TableCellData>,
        String,
    )>,
    // #[prop(optional, default = Callback::new(|_| {}))] on_row_delete: Callback<
    //     HashMap<String, TableCellData>,
    // >,
    #[prop(default = false, optional)] editable: bool,
    #[prop(default = false, optional)] deletable: bool,
) -> impl IntoView {
    let props = Memo::new(move |_| TableProps {
        columns: data.get().0,
        data: data.get().1,
        page_size,
        on_row_click,
        on_row_action,
        editable,
        deletable,
    });

    let (current_page, set_current_page) = signal(1);
    let (sorted_column_info, set_sorted_column_info) = signal(String::new());

    // Derived signal for paginated data
    let pagination_state = Memo::new(move |_| props.get().paginate(current_page.get()));
    let current_page_total_pages = Memo::new(move |_| {
        let derived_state = pagination_state.get();
        (derived_state.0, derived_state.1)
    });

    let offset_rows = Memo::new(move |_| {
        let no_of_rows = pagination_state.get().2.len();

        if no_of_rows > 0 && no_of_rows < page_size {
            page_size - no_of_rows
        } else {
            0
        }
    });

    let on_page_change = Callback::new(move |page: usize| {
        set_current_page.set(page);
    });

    let on_click_sort = Callback::new(move |mut column: Column| {
        if !column.sortable {
            return;
        };
        column.toggle_sort().toggle_sort_icon();
        let sorted_data = props.get().sort(&column);
        let mut updated_columns = props.get().columns;
        if let Some(c) = updated_columns.iter_mut().find(|c| c.name == column.name) {
            set_sorted_column_info.set(format!("-{}-{:?}", column.name, column.sort_order));
            *c = column.clone();
        }

        set_current_page.set(1);
        data.set((updated_columns, sorted_data));
    });

    let on_click_row_handler = move |row_data: HashMap<String, TableCellData>| {
        props.get().on_row_click.run(row_data);
    };

    let on_click_action_handler =
        move |(row_data, action_type): (HashMap<String, TableCellData>, String)| {
            Callback::new(move |_| {
                let action_type = action_type.clone();
                let row_data = row_data.clone();

                props.get().on_row_action.run((row_data, action_type));
            })
        };

    view! {
        <div class="w-full flex flex-col justify-between">
            <div class="overflow-x-auto">
                <table class="table-fixed border-separate border border-light-gray rounded-[5px] table-fixed min-w-full h-full mt-4 mb-4 text-md">
                    <thead>
                        <tr class="p-2">
                            <For
                                each=move || props.get().columns
                                key=|column| format!("{}-{:?}", column.name.clone(), column.sort_order)
                                let (column)
                            >
                                <th
                                    class="border-b p-2 border-light-gray text-nowrap font-bold text-left cursor-pointer min-w-[150px]"
                                    on:click=move |_| on_click_sort.run(column.clone())
                                >
                                    <span class="flex flex-row items-center">
                                        <span>{column.name.clone()}</span>
                                        {if column.sortable {
                                            Some(view! {
                                                <span class="text-primary">
                                                    { column.sort_icon.run() }
                                                </span>
                                            })
                                        } else {
                                            None
                                        }}
                                    </span>
                                </th>
                            </For>
                            {move || if props.get().editable || props.get().deletable {
                                Some(view! {
                                    <th class="border-b p-2 border-light-gray text-wrap font-bold text-left">
                                        "Actions"
                                    </th>
                                })
                            } else {
                                None
                            }}
                        </tr>
                    </thead>
                    <tbody>
                        <For
                            each=move || pagination_state.get().2
                            key=move |row| match row.get("id").clone() {
                                Some(TableCellData::String(s)) => format!("{}{}", s.clone(), sorted_column_info.get()),
                                _ => String::new(),
                            }
                            let(row_data)
                        >
                            {
                                let row_data_row_click = row_data.clone();
                                let row_data_cols = row_data.clone();

                                view! {
                                    <tr
                                        class="border-b border-light-gray p-2"
                                        on:click=move |_| on_click_row_handler(row_data_row_click.clone())
                                    >
                                        // Computed row columns
                                        {
                                            let id = match row_data.get("id").clone() {
                                                Some(TableCellData::String(s)) => s.clone(),
                                                _ => String::new(),
                                            };

                                            view! {
                                                <For
                                                    each=move || props.get().columns.clone()
                                                    key=move |column| {
                                                        format!("{}-{}", column.name.clone(), id)
                                                    }
                                                    let(column)
                                                >
                                                    <td class="p-2 text-wrap">
                                                        {match row_data_cols.get(&column.name).clone() {
                                                            Some(TableCellData::String(s)) => s.clone().into_any().into_view(),
                                                            Some(TableCellData::Int32(i)) => i.to_string().into_any().into_view(),
                                                            Some(TableCellData::Int64(i)) => i.to_string().into_any().into_view(),
                                                            Some(TableCellData::Usize(u)) => u.to_string().into_any().into_view(),
                                                            Some(TableCellData::UInt32(u)) => u.to_string().into_any().into_view(),
                                                            Some(TableCellData::UInt64(u)) => u.to_string().into_any().into_view(),
                                                            Some(TableCellData::UInt128(u)) => u.to_string().into_any().into_view(),
                                                            Some(TableCellData::Html(html)) => html.run().into_view(),
                                                            Some(TableCellData::Float32(f)) => format!("{:.2}", f).into_any().into_view(),
                                                            Some(TableCellData::Float64(f)) => format!("{:.2}", f).into_any().into_view(),
                                                            Some(TableCellData::Bool(b)) => b.to_string().into_any().into_view(),
                                                            Some(TableCellData::DateTime(dt)) => {
                                                                match DateTime::parse_from_rfc3339(dt) {
                                                                    Ok(dt) => dt.format("%d %b %Y").to_string().into_any().into_view(),
                                                                    Err(_) => "Invalid Date".into_any().into_view(),
                                                                }
                                                            },
                                                            Some(TableCellData::Duration(dt)) => {
                                                                let utc: DateTime<Utc> = Utc::now();

                                                                get_elapsed_time(dt, &utc).into_any().into_view()
                                                            },
                                                            None => "N/A".into_any().into_view(),
                                                        }}
                                                    </td>
                                                </For>
                                                // Action columns
                                                {if props.get().editable || props.get().deletable {
                                                    let showing = RwSignal::new(false);

                                                    Some(view! {
                                                        <td class="flex flex-row items-center gap-2 h-full py-2">
                                                            <Popover showing=showing display_item=|| view!{
                                                                <BasicButton
                                                                                icon=Some(BsThreeDots)
                                                                            />
                                                                }>
                                                                <div class="flex flex-col gap-2">
                                                                    {if props.get().editable {
                                                                        Some(view! {
                                                                            <BasicButton
                                                                                style_ext="px-0 hover:bg-primary hover:text-contrast-white"
                                                                                onclick=on_click_action_handler((row_data.clone(), "edit".into()))
                                                                                >
                                                                                <span class="flex items-center justify-between">
                                                                                    <span>Edit</span>
                                                                                    <Icon icon=BsPencil />
                                                                                </span>
                                                                            </BasicButton>
                                                                        })
                                                                    } else {
                                                                        None
                                                                    }}
                                                                    {if props.get().deletable {
                                                                        Some(view! {
                                                                            <BasicButton
                                                                                style_ext="text-danger px-0 hover:bg-danger hover:text-contrast-white"
                                                                                onclick=on_click_action_handler((row_data.clone(), "delete".into()))
                                                                                >
                                                                                <span class="flex items-center justify-between">
                                                                                    <span>Delete</span>
                                                                                    <Icon icon=BsTrash />
                                                                                </span>
                                                                            </BasicButton>
                                                                        })
                                                                    } else {
                                                                        None
                                                                    }}
                                                                </div>
                                                            </Popover>
                                                        </td>
                                                    })
                                                } else {
                                                    None
                                                }}
                                            }
                                        }
                                    </tr>
                                }
                            }
                        </For>
                        {
                            move || if offset_rows.get() > 0 {
                                let blank_rows = (0..offset_rows.get()).collect::<Vec<usize>>();

                                Some(
                                    view!{
                                        <For
                                            each=move || blank_rows.clone()
                                            key=move |row| row.to_string()

                                            let(_)
                                        >
                                            {
                                                view! {
                                                    <tr class="border-b border-light-gray">
                                                        <td class="p-[24px]" colspan={props.get().columns.len()}>""</td>
                                                    </tr>
                                                }
                                            }
                                        </For>
                                    }
                                )
                            } else {
                                None
                            }
                        }
                        {move || if pagination_state.get().2.is_empty() {
                            Some(view! {
                                <tr>
                                    <td colspan={props.get().columns.len() + 1}>
                                        <div class="py-2 flex items-center justify-center">
                                            <div class="flex-1 flex flex-col items-center justify-center">
                                                <Icon width="2em" height="2em" icon=ImDrawer2 />
                                                <p>"No Content"</p>
                                            </div>
                                        </div>
                                    </td>
                                </tr>
                            })
                        } else {
                            None
                        }}
                    </tbody>
                </table>
            </div>
            <Pagination
                pagination_state={current_page_total_pages}
                on_page_change={on_page_change}
            />
        </div>
    }.into_any()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    // SortOrder

    #[test]
    fn sort_order_default() {
        assert_eq!(SortOrder::default(), SortOrder::Default);
    }

    // Column::toggle_sort

    #[test]
    fn toggle_sort_cycles_correctly() {
        let mut col = Column::new("Name", true);
        assert_eq!(col.sort_order, SortOrder::Default);

        col.toggle_sort();
        assert_eq!(col.sort_order, SortOrder::Ascending);

        col.toggle_sort();
        assert_eq!(col.sort_order, SortOrder::Descending);

        col.toggle_sort();
        assert_eq!(col.sort_order, SortOrder::Default);
    }

    #[test]
    fn non_sortable_column_still_toggles_sort_order() {
        // sortable flag is enforced by the click handler, not Column itself
        let mut col = Column::new("Name", false);
        col.toggle_sort();
        assert_eq!(col.sort_order, SortOrder::Ascending);
    }

    #[test]
    fn column_eq_based_on_name_only() {
        let a = Column::new("Age", true);
        let mut b = Column::new("Age", true);
        b.toggle_sort();
        assert_eq!(a, b); // sort_order difference ignored
    }

    #[test]
    fn columns_with_different_names_not_equal() {
        let a = Column::new("Age", true);
        let b = Column::new("Name", true);
        assert_ne!(a, b);
    }

    // TableCellData::PartialEq

    #[test]
    fn string_cells_equal() {
        assert_eq!(
            TableCellData::String("hello".into()),
            TableCellData::String("hello".into())
        );
    }

    #[test]
    fn string_cells_not_equal() {
        assert_ne!(
            TableCellData::String("a".into()),
            TableCellData::String("b".into())
        );
    }

    #[test]
    fn different_variants_not_equal() {
        assert_ne!(TableCellData::String("1".into()), TableCellData::Int32(1));
    }

    #[test]
    fn datetime_cells_equal_by_timestamp() {
        let a = TableCellData::DateTime("2024-01-01T00:00:00Z".into());
        let b = TableCellData::DateTime("2024-01-01T00:00:00Z".into());
        assert_eq!(a, b);
    }

    #[test]
    fn datetime_cells_not_equal_for_different_dates() {
        let a = TableCellData::DateTime("2024-01-01T00:00:00Z".into());
        let b = TableCellData::DateTime("2024-06-01T00:00:00Z".into());
        assert_ne!(a, b);
    }

    #[test]
    fn html_cells_never_equal() {
        let a = TableCellData::Html(ViewFn::from(|| view! {}));
        let b = TableCellData::Html(ViewFn::from(|| view! {}));
        assert_ne!(a, b);
    }

    #[test]
    fn bool_cells_equal() {
        assert_eq!(TableCellData::Bool(true), TableCellData::Bool(true));
        assert_ne!(TableCellData::Bool(true), TableCellData::Bool(false));
    }

    // TableProps::paginate

    fn make_rows(n: usize) -> Vec<HashMap<String, TableCellData>> {
        (0..n)
            .map(|i| {
                let mut row = HashMap::new();
                row.insert("id".to_string(), TableCellData::String(i.to_string()));
                row
            })
            .collect()
    }

    fn make_props(rows: usize, page_size: usize) -> TableProps {
        TableProps {
            columns: vec![Column::new("id", false)],
            data: make_rows(rows),
            page_size,
            on_row_click: Callback::new(|_| {}),
            on_row_action: Callback::new(|_| {}),
            editable: false,
            deletable: false,
        }
    }

    #[test]
    fn paginate_first_page() {
        let mut props = make_props(25, 10);
        let (page, total_pages, rows) = props.paginate(1);
        assert_eq!(page, 1);
        assert_eq!(total_pages, 3);
        assert_eq!(rows.len(), 10);
    }

    #[test]
    fn paginate_last_page_partial() {
        let mut props = make_props(25, 10);
        let (_, _, rows) = props.paginate(3);
        assert_eq!(rows.len(), 5);
    }

    #[test]
    fn paginate_exact_multiple() {
        let mut props = make_props(20, 10);
        let (_, total_pages, _) = props.paginate(1);
        assert_eq!(total_pages, 2);
    }

    #[test]
    fn paginate_single_page() {
        let mut props = make_props(5, 10);
        let (_, total_pages, rows) = props.paginate(1);
        assert_eq!(total_pages, 1);
        assert_eq!(rows.len(), 5);
    }

    #[test]
    fn paginate_empty_data() {
        let mut props = make_props(0, 10);
        let (_, total_pages, rows) = props.paginate(1);
        assert_eq!(total_pages, 0);
        assert_eq!(rows.len(), 0);
    }

    // TableProps::sort

    fn make_string_props(values: Vec<&str>) -> TableProps {
        let rows = values
            .into_iter()
            .enumerate()
            .map(|(i, v)| {
                let mut row = HashMap::new();
                row.insert("id".to_string(), TableCellData::String(i.to_string()));
                row.insert("Name".to_string(), TableCellData::String(v.to_string()));
                row
            })
            .collect();

        TableProps {
            columns: vec![Column::new("Name", true)],
            data: rows,
            page_size: 10,
            on_row_click: Callback::new(|_| {}),
            on_row_action: Callback::new(|_| {}),
            editable: false,
            deletable: false,
        }
    }

    fn get_name(row: &HashMap<String, TableCellData>) -> &str {
        match row.get("Name") {
            Some(TableCellData::String(s)) => s.as_str(),
            _ => "",
        }
    }

    #[test]
    fn sort_ascending_strings() {
        let mut props = make_string_props(vec!["Charlie", "Alice", "Bob"]);
        let mut col = Column::new("Name", true);
        col.toggle_sort(); // Ascending
        let sorted = props.sort(&col);
        assert_eq!(get_name(&sorted[0]), "Alice");
        assert_eq!(get_name(&sorted[1]), "Bob");
        assert_eq!(get_name(&sorted[2]), "Charlie");
    }

    #[test]
    fn sort_descending_strings() {
        let mut props = make_string_props(vec!["Charlie", "Alice", "Bob"]);
        let mut col = Column::new("Name", true);
        col.toggle_sort(); // Ascending
        col.toggle_sort(); // Descending
        let sorted = props.sort(&col);
        assert_eq!(get_name(&sorted[0]), "Charlie");
        assert_eq!(get_name(&sorted[1]), "Bob");
        assert_eq!(get_name(&sorted[2]), "Alice");
    }

    #[test]
    fn sort_default_order_does_not_change_data() {
        let mut props = make_string_props(vec!["Charlie", "Alice", "Bob"]);
        let col = Column::new("Name", true); // SortOrder::Default
        let sorted = props.sort(&col);
        assert_eq!(get_name(&sorted[0]), "Charlie");
        assert_eq!(get_name(&sorted[1]), "Alice");
        assert_eq!(get_name(&sorted[2]), "Bob");
    }

    // TableProps::PartialEq

    #[test]
    fn table_props_equal_ignores_callbacks() {
        let a = make_props(5, 10);
        let b = make_props(5, 10);
        assert_eq!(a, b);
    }

    #[test]
    fn table_props_not_equal_different_page_size() {
        let a = make_props(5, 10);
        let b = make_props(5, 5);
        assert_ne!(a, b);
    }
}
