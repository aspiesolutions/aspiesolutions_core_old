// I want a way to encode a SQL query based on this crate
// without having to pull in the entire sea orm infrastructure if possible

// each type of data can have many different operations on them

// dates : before, after between (range)
// numbers: less than, greater than, equal to, between (range)
// strings: equals, contains
// booleans: true false

// there should be a way to have multiple sets of conditions

// ex date between x and y OR date between a and b OR date between c and d

/*
NOTE: dates can be treated like numbers
comparisons:
WHERE number IN list
WHERE number EQ number
WHERE number < number
WHERE number > number
WHERE number != number

WHERE string IN list
WHERE string like 'string'
WHERE string ilike 'string'
WHERE string contains 'string'
WHERE string == 'string'

conditions:
ANY (or) LIST
ALL (and) LIST
NOT (negate) LIST

condition=[table.column.operator=[list,list,list],table.column.operator=[list,list,list]]
any[transactions.amount.bti=[0,100],transactions.amount.bti=[200,300]]
q=table.column.operator=value

*/
// pub const ASSIGNMENT_SEPERATOR: &str = "=";
// pub const IDENTIFIER_SEPERATOR:&str=".";
// pub const LIST_BEGIN: &str = "[";
// pub const LIST_END: &str = "]";
// pub const LIST_ITEM_SEPERATOR: &str = ",";

// pub const OPERATOR_IN:&str = "in";
// pub const OPERATOR_GREATER_THAN:&str="gt";
// pub const OPERATOR_GREATER_THAN_OR_EQUAL:&str="ge";
// pub const OPERATOR_LESS_THAN:&str="lt";
// pub const OPERATOR_LESS_THAN_OR_EQUAL:&str="le";
// pub const OPERATOR_EQUALS:&str="eq";
// pub const OPERATOR_NOT_EQUALS:&str = "ne";
// pub const OPERATOR_IS_NULL:&str="is_null";
// pub const OPERATOR_IS_NOT_NULL:&str = "is_not_null";
