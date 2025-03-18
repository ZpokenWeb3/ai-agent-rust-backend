use serde::Serialize;

#[derive(Serialize)]
pub struct PaginatedResponse<T> {
    page_size: usize,
    page_number: usize,
    total_items: usize,
    total_pages: usize,
    data: Vec<T>,
}

impl<T> PaginatedResponse<T> {
    pub fn new_paginated_response(page_size: usize, page_number: usize, total_items: usize, data: Vec<T>) -> Self {
        let total_pages = (total_items as f64 / page_size as f64).ceil() as usize;
        Self {
            page_size,
            page_number,
            total_items,
            total_pages,
            data,
        }
    }
}
