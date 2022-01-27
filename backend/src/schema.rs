table! {
    entries (id) {
        id -> Integer,
        item -> Integer,
        list -> Integer,
        amount -> Integer,
    }
}

table! {
    items (id) {
        id -> Integer,
        name -> Varchar,
        price -> Float,
    }
}

table! {
    lists (id) {
        id -> Integer,
        name -> Varchar,
    }
}

table! {
    users (id) {
        id -> Integer,
        username -> Varchar,
        password -> Varchar,
    }
}

table! {
    users_to_lists (id) {
        id -> Integer,
        list -> Integer,
        user -> Integer,
    }
}

allow_tables_to_appear_in_same_query!(
    entries,
    items,
    lists,
    users,
    users_to_lists,
);
