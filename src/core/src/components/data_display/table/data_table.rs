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

#[derive(Clone)]
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

/// This component is a data table that can be used to display data in a table format.
/// It provides sorting, pagination, and row actions such as editing and deleting.
/// Example usage:
/// ```
/// // Data preparation:
/// let columns = vec![
///    Column::new("Transaction ID", false),
///    Column::new("Date", true),
///  ];
///
/// // Assuming that transactions variable is a vector of Transaction structs probably deserialized from a backend API JSON response
/// // Use the TableCellData enum to format table cell values to the variants you want.
/// let transactions = transactions
///    .iter()
///    .map(|transaction| {
///        let mut hash_map_data = HashMap::new();
///
///        // This id is the unique identifier of the table row. and is a MUST for the table to function properly.
///        // *Note:* The id is a MUST for the table to function properly. You might be forced to generate a unique id for each row if your data does not have a unique identifier.
///        hash_map_data.insert(
///            "id".to_string(),
///            TableCellData::String(transaction.id.clone()),
///        );
///
///        hash_map_data.insert(
///            "Transaction ID".to_string(),
///            TableCellData::String(transaction.id.clone()),
///        );
///        hash_map_data.insert(
///            "Date".to_string(),
///            TableCellData::DateTime(transaction.date.clone()),
///        );
///        hash_map_data
///    })
///    .collect();
///
/// let table_data = RwSignal::new((columns, transactions));
///
/// <DataTable data=table_data editable=true deletable=true />
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
    }
}
