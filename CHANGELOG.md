# 2.2.1

- Fix derived `array_compatible` implementation

# 2.2.0

- Implement `PartialEq` and `Eq` on `BoxedRawValue`. This does not parse the JSON. Rather it does a simple check that the serialized bytes are identical, so it
    will fail on some cases, such as when the keys of an object are serialized in a different order. This trait implementation can be
    disabled by turning off the `boxed_raw_value_eq` feature.
- The `sqlx_json_decode` macro now also implements the `PgHasArrayType` trait for the given type.

# 2.1.0

- Add `Encode` trait implementation for `BoxedRawValue`

# 2.0.1

- BoxedRawValue: Implement in_referenceable = false in JsonSchema trait impl

# 2.0.0

- Support sqlx 0.7
- Add `BoxedRawValue` to simplify reading Box<RawValue> from sqlx queries.

# 1.0.0
 
Initial Release
