use super::error::PaginationError;
use serde::{Deserialize, Serialize};

const DEFAULT_OFFSET: usize = 0;
const DEFAULT_RESULTS: usize = 10;
const MAX_RESULTS: usize = 100;

/// Pagination Struct to represent a single
/// page in a user-based query
///
/// The pagination is used in get_* queries, and
/// represents the user-input values. There is another
/// structure, the [ValidatedPagination] that represents
/// the pagination fields after they have been validated
/// to ensure they exist, and provides sane defaults for the
/// values
///
/// # Example Query
/// GET requests to a route tha has a pagination attached:
/// `/questions?offset=0&max_results=25`
///
/// There is a maximum value for max_results, at [MAX_RESULTS].
///
/// Additionally, if values are not supplied, defaults are taken:
/// - offset: [DEFAULT_OFFSET]
/// - max_results: [MAX_RESULTS]
#[derive(Debug, Clone, Copy, Deserialize)]
pub struct Pagination {
    pub offset: Option<usize>,
    pub max_results: Option<usize>,
}

impl Pagination {
    /// Validates the user-input Pagination struct, returning a ValidatedPagination struct if
    /// successful, or producing a PaginationError if there is an issue.
    ///
    /// ```
    ///     use stack_underflow::models::pagination::Pagination;
    ///     let pagination = Pagination { offset: Some(10), max_results: Some(10_000_000) };
    ///     // Exceed maximum size
    ///     assert!(pagination.validate().is_err());
    ///
    ///     let pagination = Pagination { offset: None, max_results: None }.validate();
    ///     assert!(pagination.is_ok());
    ///     assert_eq!(pagination.unwrap().offset, 0);
    /// ```
    pub fn validate(self) -> Result<ValidatedPagination, PaginationError> {
        Ok(ValidatedPagination {
            offset: self.offset.unwrap_or(DEFAULT_OFFSET),
            max_results: match self.max_results {
                None => DEFAULT_RESULTS,
                Some(max_results) if max_results > MAX_RESULTS => {
                    return Err(PaginationError::MaximumPageSizeExceeded(max_results))
                }
                Some(max_results) => max_results,
            },
        })
    }
}

/// Represents a pagination struct that has been validated from user input. As a result, the
/// ValidatedPagination struct is safe to use in other contexts.
///
/// ## Example:
/// ```
///     use stack_underflow::models::pagination::Pagination;
///     let pagination = Pagination { offset: Some(0), max_results: Some(10) }.validate().unwrap();
///
///     assert_eq!(pagination.offset, 0);
///     assert_eq!(pagination.max_results, 10);
///
#[derive(Debug, Clone, Copy, Serialize)]
pub struct ValidatedPagination {
    pub offset: usize,
    pub max_results: usize,
}

/// Represents a response to the customer that has been paginated. The PaginatedResponse provides
/// the customer enough context to create another paged request, so that the client has context on
/// which page it is processing, if for example it queries for more than one page at once.
///
/// The paginated response is generic over the type of queried item returned, so as to allow this
/// reponse context to be shared by any paginated response. Generally speaking, if a get_* request
/// takes a Pagination as input, it should return a PaginatedResponse as output.
#[derive(Debug, Clone, Serialize)]
pub struct PaginatedResponse<T: Serialize> {
    #[serde(rename(serialize = "paginationContext"))]
    pub page: ValidatedPagination,
    pub items: Vec<T>,
}

impl<T> PaginatedResponse<T>
where
    T: Serialize,
{
    pub fn new(page: ValidatedPagination, items: Vec<T>) -> Self {
        Self { page, items }
    }
}
