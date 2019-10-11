# Types

This directory has our custom types.

We generally prefer custom types that are type aliases (vs. structs)


## Numbers

For types that are real numbers, we generally prefer implementation
using decimals (vs. floating points) because we prefer stability and
transmissabily (vs. calculation speed).

Some of our use cases:

  * Currency, such as financial transactions in nano-dollars.

  * Probability, such as a range of 0 to 1.

  * Geography, such as latitude, longitude, etc.

  * Time, such as in nanosecond-precision.

Our current implementation uses the BigDecimal crate.
