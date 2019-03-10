//! This crate provides a client for interacting with the crossref-api
//!
//! [Crossref API docs](https://github.com/CrossRef/rest-api-doc>)
//! `Crossref` - Crossref search API. The `Crossref` crate provides methods matching Crossref API routes:

//! * `works` - `/works` route
//! * `members` - `/members` route
//! * `prefixes` - `/prefixes` route
//! * `funders` - `/funders` route
//! * `journals` - `/journals` route
//! * `types` - `/types` route
//! * `agency` - `/works/{doi}/agency` get DOI minting agency
//!
//! ## Usage

//! ### Create a `Crossref` client:

//! ```edition2018
//! # use crossref::Crossref;
//! # fn run() -> Result<(), crossref::Error> {
//! let client = Crossref::builder().build()?;
//! # Ok(())
//! # }
//! ```
//!
//! If you have an [Authorization token for Crossref's Plus service](https://github.com/CrossRef/rest-api-doc#authorization-token-for-plus-service):
//!
//! ```edition2018
//! # use crossref::Crossref;
//! # fn run() -> Result<(), crossref::Error> {
//! let client = Crossref::builder()
//! .token("token")
//! .build()?;
//! # Ok(())
//! # }
//! ```
//!
//! Encouraged to use the **The Polite Pool**:
//!
//! [Good manners = more reliable service](https://github.com/CrossRef/rest-api-doc#good-manners--more-reliable-service)
//!
//! To get into Crossref's polite pool include a email address
//!
//! ```edition2018
//! # use crossref::Crossref;
//! # fn run() -> Result<(), crossref::Error> {
//! let client = Crossref::builder()
//!     .polite("polite@example.com")
//!     .token("your token")
//!     .build()?;
//! # Ok(())
//! # }
//! ```
//!
//! ### Constructing Queries
//! Not all components support queries and there are custom available parameters for each route that supports querying.
//! For each resource components that supports querying there exist a Query struct: `WorksQuery`, `MembersQuery`, `FundersQuery`. The `WorksQuery` also differs from the others by supporting [deep paging with cursors](https://github.com/CrossRef/rest-api-doc#deep-paging-with-cursors) and [field queries](https://github.com/CrossRef/rest-api-doc#works-field-queries).
//!
//! Otherwise creating queries works the same for all resource components:
//!
//! ```edition2018
//! # use crossref::*;
//! # fn run() -> Result<()> {
//! let query = WorksQuery::new()
//! .query("Machine Learning")
//! // field queries supported for `Works`
//! .field_query(FieldQuery::author("Some Author"))
//! // filters are specific for each resource component
//! .filter(WorksFilter::HasOrcid)
//! .order(Order::Asc)
//! .sort(Sort::Score);
//! # Ok(())
//! # }
//! ```
//!
//!
//! ### Get Records
//!
//! See [this table](https://github.com/CrossRef/rest-api-doc#resource-components) for a detailed overview of the major components.
//!
//! There are 3 available targets:
//!
//! * **standalone resource components**: `/works`, `/members`, etc. that return a list list of the corresponding items and can be specified with queries
//! * **Resource component with identifiers**: `/works/{doi}?<query>`,`/members/{member_id}?<query>`, etc. that returns a single item if found.
//! * **combined with the `works` route**: The works component can be appended to other resources: `/members/{member_id}/works?<query>` etc. that returns a list of matching `Work` items.
//!
//! This resembles in the enums of the resource components, eg. for `Members`:
//!
//! ```edition2018
//! # use crossref::query::*;
//! pub enum Members {
//!     /// target a specific member at `/members/{id}`
//!     Identifier(String),
//!     /// target all members that match the query at `/members?query...`
//!     Query(MembersQuery),
//!     /// target a `Work` for a specific funder at `/members/{id}/works?query..`
//!     Works(WorksCombined),
//! }
//! ```
//!
//! All options are supported by the client:
//!
//! **Single Item by DOI (ID)**
//!
//! Analogous methods exist for all resource components
//!
//! ```edition2018
//! # use crossref::*;
//! # fn run() -> Result<()> {
//! # let client = Crossref::builder().build()?;
//! let work = client.work("10.1037/0003-066X.59.1.29")?;
//!
//! let agency = client.work_agency("10.1037/0003-066X.59.1.29")?;
//!
//! let funder = client.funder("funder_id")?;
//!
//! let member = client.member("member_id")?;
//! # Ok(())
//! # }
//! ```
//!
//! **Query**
//!
//! ```edition2018
//! # use crossref::*;
//! # fn run() -> Result<()> {
//! # let client = Crossref::builder().build()?;
//! let query = WorksQuery::new().query("Machine Learning");
//!
//! // one page of the matching results
//! let works = client.works(query)?;
//! # Ok(())
//! # }
//! ```
//!
//! **Convenience method for Work Items by terms**
//!
//! ```edition2018
//! # use crossref::*;
//! # fn run() -> Result<()> {
//! # let client = Crossref::builder().build()?;
//! let works = client.query_works("Machine Learning")?;
//! # Ok(())
//! # }
//! ```
//!
//! **Combining Routes with the `Works` route**
//!
//! For each resource component other than `Works` there exist methods to append a `WorksQuery` with the ID option `/members/{member_id}/works?<query>?`
//!
//! ```rustnorun
//! # use crossref::*;
//! # fn run() -> Result<()> {
//! # let client = Crossref::builder().build()?;
//! let works = client.member_works("member_id", WorksQuery::new()
//! .query("machine learning")
//! .sort(Sort::Score))?;
//! # Ok(())
//! # }
//! ```
//!
//! Convenience method to append works query term:
//!
//! ```edition2018
//! # use crossref::*;
//! # fn run() -> Result<()> {
//! # let client = Crossref::builder().build()?;
//! let works = client.member_works("member id", "Machine Learning")?;
//! # Ok(())
//! # }
//! ```
//!
//! **Deep paging for Works**
//! ```edition2018
//! use crossref::{Crossref, WorksQuery, WorksFilter};
//! # fn run() -> Result<(), crossref::Error> {
//! let client = Crossref::builder().build()?;
//!
//! // request a next-cursor first
//! let query = WorksQuery::new()
//!     .query("Machine Learning")
//!     .new_cursor();
//!
//! let works = client.works(query.clone())?;
//! # Ok(())
//! # }
//! ```
//!
//!
//#![deny(warnings)]
#![deny(missing_docs)]
#![allow(unused)]
#[macro_use]
extern crate serde_derive;

mod error;
/// provides types to construct a specific query
pub mod query;
/// provides the response types of the crossref api
pub mod response;

/// an async client
#[cfg(feature = "client")]
pub mod client;

#[doc(inline)]
pub use self::error::{Error, Result};

#[doc(inline)]
pub use self::query::works::{
    FieldQuery, WorkResultControl, Works, WorksCombined, WorksFilter, WorksQuery,
};
#[doc(inline)]
pub use self::query::{CrossrefQuery, CrossrefRoute, Order, Sort};

pub use self::query::{Funders, Journals, Members, Prefixes, Type, Types};

pub use self::response::{
    CrossrefType, Funder, FunderList, Journal, JournalList, Member, MemberList, TypeList, Work,
    WorkAgency, WorkList,
};

pub(crate) use self::response::{Message, Response};

use crate::error::ErrorKind;
use crate::query::{FundersQuery, MembersQuery};
use crate::response::{MessageType, Prefix};
use reqwest::{self, Client};

macro_rules! get_item {
    ($ident:ident, $value:expr, $got:expr) => {
        if let Some(msg) = $value {
            match msg {
                Message::$ident(item) => Ok(item),
                _ => Err(ErrorKind::UnexpectedItem {
                    expected: MessageType::$ident,
                    got: $got,
                }
                .into()),
            }
        } else {
            Err(ErrorKind::MissingMessage {
                expected: MessageType::$ident,
            }
            .into())
        }
    };
}

/// Struct for Crossref search API methods
#[derive(Debug, Clone)]
pub struct Crossref {
    /// use another base url than `api.crossref.org`
    pub base_url: String,
    /// the reqwest client that handles the requests
    pub client: Client,
}

impl Crossref {
    const BASE_URL: &'static str = "https://api.crossref.org";

    /// Constructs a new `CrossrefBuilder`.
    ///
    /// This is the same as `Crossref::builder()`.
    pub fn builder() -> CrossrefBuilder {
        CrossrefBuilder::new()
    }

    /// Transforms the `CrossrefQuery` in the request route and  executes the request
    ///
    /// # Errors
    ///
    /// If it was a bad url, the server will return `Resource not found` a `ResourceNotFound` error will be returned in this case
    /// Also fails if the json response body could be parsed into `Response`
    /// Fails if there was an error in reqwest executing the request [::reqwest::RequestBuilder::send]
    fn get_response<T: CrossrefQuery>(&self, query: T) -> Result<Response> {
        let resp = self
            .client
            .get(&query.to_url(&self.base_url)?)
            .send()?
            .text()?;
        if resp.starts_with("Resource not found") {
            Err(ErrorKind::ResourceNotFound {
                resource: query.resource_component(),
            }
            .into())
        } else {
            Ok(serde_json::from_str(&resp)?)
        }
    }

    /// Return the `Work` items that match a certain query.
    ///
    /// To search only by query terms use the convenience query method [Crossref::query_works]
    ///
    /// # Example
    ///
    /// ```edition2018
    /// use crossref::{Crossref, WorksQuery, WorksFilter, FieldQuery};
    /// # fn run() -> Result<(), crossref::Error> {
    /// let client = Crossref::builder().build()?;
    ///
    /// let query = WorksQuery::new()
    ///     .query("Machine Learning")
    ///     .filter(WorksFilter::HasOrcid)
    ///     .order(crossref::Order::Asc)
    ///     .field_query(FieldQuery::author("Some Author"))
    ///     .sort(crossref::Sort::Score);
    ///
    /// let works = client.works(query)?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [Deep paging results](https://github.com/CrossRef/rest-api-doc#deep-paging-with-cursors)
    ///
    /// ```edition2018
    /// use crossref::{Crossref, WorksQuery, WorksFilter};
    /// # fn run() -> Result<(), crossref::Error> {
    /// let client = Crossref::builder().build()?;
    ///
    /// // request a next-cursor first
    /// let query = WorksQuery::new()
    ///     .query("Machine Learning")
    ///     .new_cursor();
    ///
    /// let works = client.works(query.clone())?;
    ///
    /// // this continues from where this first response stopped
    /// // if no more work items are available then a empty list will be returned
    /// let deep_works = client.works(
    ///     query.next_cursor(&works.next_cursor.unwrap())
    /// )?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// This method fails if the `works` element expands to a bad route `ResourceNotFound`
    /// Fails if the response body doesn't have `message` field `MissingMessage`.
    /// Fails if anything else than a `WorkList` is returned as message `UnexpectedItem`
    pub fn works(&self, query: WorksQuery) -> Result<WorkList> {
        let resp = self.get_response(Works::Query(query))?;
        // TODO add deep paging support
        get_item!(WorkList, resp.message, resp.message_type)
    }

    /// Return the `Work` that is identified by  the `doi`.
    ///
    /// # Errors
    /// This method fails if the doi could not identified `ResourceNotFound`
    ///
    pub fn work(&self, doi: &str) -> Result<Work> {
        let resp = self.get_response(Works::Identifier(doi.to_string()))?;
        get_item!(Work, resp.message, resp.message_type).map(|x| *x)
    }

    /// Return the `Agency` that registers the `Work` identified by  the `doi`.
    ///
    /// # Errors
    /// This method fails if the doi could not identified `ResourceNotFound`
    ///
    pub fn work_agency(&self, doi: &str) -> Result<WorkAgency> {
        let resp = self.get_response(Works::Agency(doi.to_string()))?;
        get_item!(WorkAgency, resp.message, resp.message_type)
    }

    /// Convenience method to execute [Crossref::works] with a query only consisting of terms.
    ///
    /// # Example
    ///
    /// ```edition2018
    /// # fn run() -> Result<(), crossref::Error> {
    /// let client = crossref::Crossref::builder().build()?;
    ///
    /// let works = client.query_works("Machine Learning")?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    /// This would be the same as
    ///
    /// ```edition2018
    /// use crossref::{Crossref, WorksQuery, WorksFilter};
    /// # fn run() -> Result<(), crossref::Error> {
    /// let client = Crossref::builder().build()?;
    ///
    /// let works = client.works(WorksQuery::new()
    ///        .query("Machine Learning"))?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn query_works(&self, term: &str) -> Result<WorkList> {
        self.works(WorksQuery::new().query(term))
    }

    /// Return the matching `Funders` items.
    pub fn funders(&self, funders: FundersQuery) -> Result<FunderList> {
        let resp = self.get_response(Funders::Query(funders))?;
        get_item!(FunderList, resp.message, resp.message_type)
    }

    /// Return the `Funder` for the `id`
    pub fn funder(&self, id: &str) -> Result<Funder> {
        let resp = self.get_response(Funders::Identifier(id.to_string()))?;
        get_item!(Funder, resp.message, resp.message_type).map(|x| *x)
    }

    /// Return one page of the funder's `Work` that match the query
    pub fn funder_works(&self, funder_id: &str, term: &str) -> Result<WorkList> {
        let resp = self.get_response(Funders::Works(WorksCombined::new(
            funder_id,
            WorksQuery::new().query(term),
        )))?;
        get_item!(WorkList, resp.message, resp.message_type)
    }

    /// Return the matching `Members` items.
    pub fn members(&self, members: MembersQuery) -> Result<MemberList> {
        let resp = self.get_response(Members::Query(members))?;
        get_item!(MemberList, resp.message, resp.message_type)
    }

    /// Return the `Member` for the `id`
    pub fn member(&self, id: &str) -> Result<Member> {
        let resp = self.get_response(Members::Identifier(id.to_string()))?;
        get_item!(Member, resp.message, resp.message_type).map(|x| *x)
    }

    /// Return one page of the member's `Work` that match the query
    pub fn member_works(&self, member_id: &str, term: &str) -> Result<WorkList> {
        let resp = self.get_response(Members::Works(WorksCombined::new(
            member_id,
            WorksQuery::new().query(term),
        )))?;
        get_item!(WorkList, resp.message, resp.message_type)
    }

    /// Return the `Prefix` for the `id`
    pub fn prefix(&self, id: &str) -> Result<Prefix> {
        let resp = self.get_response(Prefixes::Identifier(id.to_string()))?;
        get_item!(Prefix, resp.message, resp.message_type)
    }

    /// Return one page of the prefix's `Work` items that match the query
    pub fn prefix_works(&self, prefix_id: &str, term: &str) -> Result<WorkList> {
        let resp = self.get_response(Prefixes::Works(WorksCombined::new(
            prefix_id,
            WorksQuery::new().query(term),
        )))?;
        get_item!(WorkList, resp.message, resp.message_type)
    }

    /// Return all available `Type`
    pub fn types(&self) -> Result<TypeList> {
        let resp = self.get_response(Types::All)?;
        get_item!(TypeList, resp.message, resp.message_type)
    }

    /// Return the `Type` for the `id`
    pub fn type_(&self, id: &str) -> Result<CrossrefType> {
        let resp = self.get_response(Types::Identifier(id.to_string()))?;
        get_item!(Type, resp.message, resp.message_type)
    }

    /// Return one page of the types's `Work` items that match the query
    pub fn type_works(&self, type_: Type, term: &str) -> Result<WorkList> {
        let resp = self.get_response(Types::Works(WorksCombined::new(
            type_.id(),
            WorksQuery::new().query(term),
        )))?;
        get_item!(WorkList, resp.message, resp.message_type)
    }
}

/// A `CrossrefBuilder` can be used to create `Crossref` with additional config.
///
/// # Example
///
/// ```edition2018
/// use crossref::Crossref;
/// # fn run() -> Result<(), crossref::Error> {
///
/// let client = Crossref::builder()
///     .polite("polite@example.com")
///     .token("your token")
///     .build()?;
/// # Ok(())
/// # }
/// ```
#[derive(Default)]
pub struct CrossrefBuilder {
    /// [Good manners = more reliable service.](https://github.com/CrossRef/rest-api-doc#good-manners--more-reliable-service)
    ///
    /// will add a `User-Agent` header by default with with the `email` email.
    /// crossref can contact you if your script misbehaves
    /// this will get you directed to the "polite pool"
    user_agent: Option<String>,
    /// the token for the Crossref Plus service will be included as `Authorization` header
    /// This token will ensure that said requests get directed to a pool of machines that are reserved for "Plus" SLA users.
    plus_token: Option<String>,
    /// use a different base url than `Crossref::BASE_URL` https://api.crossref.org
    base_url: Option<String>,
}

impl CrossrefBuilder {
    /// Constructs a new `CrossrefBuilder`.
    ///
    /// This is the same as `Crossref::builder()`.
    pub fn new() -> CrossrefBuilder {
        CrossrefBuilder::default()
    }

    /// be polite and set your email as `User-Agent`
    /// will get you in the polite pool of crossref
    pub fn polite(mut self, email: &str) -> Self {
        self.user_agent = Some(format!("mailto:{}", email));
        self
    }

    /// set the user agent directly
    pub fn user_agent(mut self, user_agent: &str) -> Self {
        self.user_agent = Some(user_agent.to_string());
        self
    }

    /// set a crossref plus service  API token
    pub fn token(mut self, token: &str) -> Self {
        self.plus_token = Some(token.to_string());
        self
    }

    /// Returns a `Crossref` that uses this `CrossrefBuilder` configuration.
    /// # Errors
    ///
    /// This will fail if TLS backend cannot be initialized see [reqwest::ClientBuilder::build]
    pub fn build(self) -> Result<Crossref> {
        use reqwest::header;
        let mut headers = header::HeaderMap::new();
        if let Some(agent) = &self.user_agent {
            headers.insert(
                header::USER_AGENT,
                header::HeaderValue::from_str(agent).map_err(|_| ErrorKind::Config {
                    msg: format!("failed to create User Agent header for `{}`", agent),
                })?,
            );
        }
        if let Some(token) = &self.plus_token {
            headers.insert(
                header::AUTHORIZATION,
                header::HeaderValue::from_str(token).map_err(|_| ErrorKind::Config {
                    msg: format!("failed to create AUTHORIZATION header for `{}`", token),
                })?,
            );
        }
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .map_err(|_| ErrorKind::Config {
                msg: "failed to initialize TLS backend".to_string(),
            })?;

        Ok(Crossref {
            base_url: self
                .base_url
                .unwrap_or_else(|| Crossref::BASE_URL.to_string()),
            client,
        })
    }
}
