use std::rc::Rc;
use yew::prelude::*;
use crate::components::icons::{
    icon_chevron_left, icon_chevron_right,
    icon_chevron_up, icon_chevron_down, icon_chevrons_up_down,
};
use crate::components::pagination::{Pagination, PaginationEllipsis, PaginationItem};
use crate::components::table::{
    Table, TableBody, TableCell, TableHead, TableHeader, TableRow,
};

// ---------------------------------------------------------------------------
// DataTable — positional rows, sortable columns
// ---------------------------------------------------------------------------

#[derive(Clone, PartialEq)]
pub struct TableCol {
    pub label: AttrValue,
    pub sortable: bool,
}

impl TableCol {
    pub fn new(label: impl Into<AttrValue>) -> Self {
        Self { label: label.into(), sortable: false }
    }
    pub fn sortable(mut self) -> Self {
        self.sortable = true;
        self
    }
}

#[derive(Properties, PartialEq)]
pub struct DataTableProps {
    pub columns: Rc<Vec<TableCol>>,
    pub rows: Rc<Vec<Vec<AttrValue>>>,
    #[prop_or_default]
    pub class: Classes,
}

pub enum DataTableMsg {
    Sort(usize),
}

pub struct DataTable {
    sort_col: Option<usize>,
    sort_asc: bool,
}

impl Component for DataTable {
    type Message = DataTableMsg;
    type Properties = DataTableProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self { sort_col: None, sort_asc: true }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            DataTableMsg::Sort(col) => {
                if self.sort_col == Some(col) {
                    self.sort_asc = !self.sort_asc;
                } else {
                    self.sort_col = Some(col);
                    self.sort_asc = true;
                }
                true
            }
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        ctx.props() != old_props
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let p = ctx.props();

        let mut rows: Vec<&Vec<AttrValue>> = p.rows.iter().collect();
        if let Some(col) = self.sort_col {
            let asc = self.sort_asc;
            rows.sort_by(|a, b| {
                let av = a.get(col).map(|v| v.as_str()).unwrap_or("");
                let bv = b.get(col).map(|v| v.as_str()).unwrap_or("");
                if asc { av.cmp(bv) } else { bv.cmp(av) }
            });
        }

        html! {
            <Table class={p.class.clone()}>
                <TableHeader>
                    <TableRow>
                        { for p.columns.iter().enumerate().map(|(i, col)| {
                            self.render_th(ctx, i, col)
                        }) }
                    </TableRow>
                </TableHeader>
                <TableBody>
                    { for rows.iter().map(|row| html! {
                        <TableRow>
                            { for row.iter().map(|cell| html! {
                                <TableCell>{ cell.clone() }</TableCell>
                            }) }
                        </TableRow>
                    }) }
                </TableBody>
            </Table>
        }
    }
}

impl DataTable {
    fn render_th(&self, ctx: &Context<Self>, col_idx: usize, col: &TableCol) -> Html {
        if !col.sortable {
            return html! { <TableHead>{ col.label.clone() }</TableHead> };
        }

        let on_sort = ctx.link().callback(move |_: MouseEvent| DataTableMsg::Sort(col_idx));
        let is_active = self.sort_col == Some(col_idx);
        let icon = if !is_active {
            icon_chevrons_up_down()
        } else if self.sort_asc {
            icon_chevron_up()
        } else {
            icon_chevron_down()
        };

        let th_base = "px-4 py-3 text-left text-xs font-semibold uppercase tracking-wide cursor-pointer select-none hover:text-foreground transition-colors";
        let th_color = if is_active { "text-foreground" } else { "text-foreground/60" };
        let th_cls = classes!(th_base, th_color);

        html! {
            <th class={th_cls} onclick={on_sort}>
                <span class="inline-flex items-center gap-1">
                    { &col.label }
                    { icon }
                </span>
            </th>
        }
    }
}

// ---------------------------------------------------------------------------
// JsonDataTable — key-value rows, sortable columns, optional pagination
// ---------------------------------------------------------------------------

/// Column definition for JsonDataTable.
#[derive(Clone, PartialEq)]
pub struct ColumnDef {
    pub key: AttrValue,
    pub label: AttrValue,
    pub sortable: bool,
}

impl ColumnDef {
    pub fn new(key: impl Into<AttrValue>, label: impl Into<AttrValue>) -> Self {
        Self { key: key.into(), label: label.into(), sortable: false }
    }
    pub fn sortable(mut self) -> Self {
        self.sortable = true;
        self
    }
}

/// A single row: list of (column_key, cell_value) pairs.
/// Missing keys render as empty cells; extra keys are ignored.
pub type DataRow = Vec<(AttrValue, AttrValue)>;

#[derive(Properties, PartialEq)]
pub struct JsonDataTableProps {
    pub columns: Rc<Vec<ColumnDef>>,
    pub rows: Rc<Vec<DataRow>>,
    /// Rows per page. 0 = no pagination (show all).
    #[prop_or_default]
    pub page_size: usize,
    #[prop_or_default]
    pub class: Classes,
}

pub enum JsonDataTableMsg {
    Sort(usize),
    GoTo(usize),
}

pub struct JsonDataTable {
    sort_col: Option<usize>,
    sort_asc: bool,
    page: usize,
}

impl Component for JsonDataTable {
    type Message = JsonDataTableMsg;
    type Properties = JsonDataTableProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self { sort_col: None, sort_asc: true, page: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            JsonDataTableMsg::Sort(col) => {
                if self.sort_col == Some(col) {
                    self.sort_asc = !self.sort_asc;
                } else {
                    self.sort_col = Some(col);
                    self.sort_asc = true;
                }
                self.page = 0;
                true
            }
            JsonDataTableMsg::GoTo(p) => {
                if self.page == p { return false; }
                self.page = p;
                true
            }
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        if ctx.props() != old_props {
            self.page = 0;
            return true;
        }
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let p = ctx.props();

        let mut rows: Vec<&DataRow> = p.rows.iter().collect();
        if let Some(col_idx) = self.sort_col {
            if let Some(col) = p.columns.get(col_idx) {
                let key = col.key.clone();
                let asc = self.sort_asc;
                rows.sort_by(|a, b| {
                    let av = cell_value(a, &key);
                    let bv = cell_value(b, &key);
                    if asc { av.cmp(bv) } else { bv.cmp(av) }
                });
            }
        }

        let (page_rows, total_pages) = if p.page_size > 0 {
            let total = rows.len().div_ceil(p.page_size);
            let start = self.page * p.page_size;
            let end   = (start + p.page_size).min(rows.len());
            (&rows[start..end], total)
        } else {
            (&rows[..], 0)
        };

        html! {
            <div class="space-y-3">
                <Table class={p.class.clone()}>
                    <TableHeader>
                        <TableRow>
                            { for p.columns.iter().enumerate().map(|(i, col)| {
                                self.render_th(ctx, i, col)
                            }) }
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        { for page_rows.iter().map(|row| html! {
                            <TableRow>
                                { for p.columns.iter().map(|col| {
                                    let val = cell_value(row, &col.key).clone();
                                    html! { <TableCell>{ val }</TableCell> }
                                }) }
                            </TableRow>
                        }) }
                    </TableBody>
                </Table>
                if total_pages > 1 {
                    { self.render_pagination(ctx, total_pages) }
                }
            </div>
        }
    }
}

impl JsonDataTable {
    fn render_pagination(&self, ctx: &Context<Self>, total_pages: usize) -> Html {
        let cur = self.page;

        let prev_cb = ctx.link().callback(move |_: MouseEvent| JsonDataTableMsg::GoTo(cur.saturating_sub(1)));
        let next_cb = {
            let last = total_pages - 1;
            ctx.link().callback(move |_: MouseEvent| JsonDataTableMsg::GoTo((cur + 1).min(last)))
        };

        let pages = page_sequence(cur, total_pages);

        html! {
            <div class="flex items-center justify-between text-sm text-foreground/60">
                <span>{ format!("Page {} of {}", cur + 1, total_pages) }</span>
                <Pagination>
                    <PaginationItem onclick={prev_cb} disabled={cur == 0}>
                        { icon_chevron_left() }
                    </PaginationItem>
                    { for pages.iter().map(|item| match item {
                        None => html! { <PaginationEllipsis /> },
                        Some(pg) => {
                            let pg = *pg;
                            let go = ctx.link().callback(move |_: MouseEvent| JsonDataTableMsg::GoTo(pg));
                            html! {
                                <PaginationItem active={pg == cur} onclick={go}>
                                    { pg + 1 }
                                </PaginationItem>
                            }
                        }
                    }) }
                    <PaginationItem onclick={next_cb} disabled={cur + 1 >= total_pages}>
                        { icon_chevron_right() }
                    </PaginationItem>
                </Pagination>
            </div>
        }
    }

    fn render_th(&self, ctx: &Context<Self>, col_idx: usize, col: &ColumnDef) -> Html {
        if !col.sortable {
            return html! { <TableHead>{ col.label.clone() }</TableHead> };
        }

        let on_sort = ctx.link().callback(move |_: MouseEvent| JsonDataTableMsg::Sort(col_idx));
        let is_active = self.sort_col == Some(col_idx);
        let icon = if !is_active {
            icon_chevrons_up_down()
        } else if self.sort_asc {
            icon_chevron_up()
        } else {
            icon_chevron_down()
        };

        let th_base = "px-4 py-3 text-left text-xs font-semibold uppercase tracking-wide cursor-pointer select-none hover:text-foreground transition-colors";
        let th_color = if is_active { "text-foreground" } else { "text-foreground/60" };
        let th_cls = classes!(th_base, th_color);

        html! {
            <th class={th_cls} onclick={on_sort}>
                <span class="inline-flex items-center gap-1">
                    { &col.label }
                    { icon }
                </span>
            </th>
        }
    }
}

fn cell_value<'a>(row: &'a DataRow, key: &AttrValue) -> &'a AttrValue {
    row.iter()
        .find(|(k, _)| k == key)
        .map(|(_, v)| v)
        .unwrap_or(&AttrValue::Static(""))
}

/// Returns a page number sequence with None for ellipsis gaps.
/// Always includes first page, last page, current ± 1.
fn page_sequence(current: usize, total: usize) -> Vec<Option<usize>> {
    let mut set: Vec<usize> = vec![0, total - 1];
    if current > 0 { set.push(current - 1); }
    set.push(current);
    if current + 1 < total { set.push(current + 1); }
    set.sort_unstable();
    set.dedup();

    let mut result = Vec::new();
    let mut prev: Option<usize> = None;
    for &pg in &set {
        if let Some(p) = prev {
            if pg > p + 1 { result.push(None); }
        }
        result.push(Some(pg));
        prev = Some(pg);
    }
    result
}
