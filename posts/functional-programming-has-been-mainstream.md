+++
title = 'Functional Programming Has Been Mainstream for Years'
description = 'Functional programming did not replace object-oriented programming. Instead, its most useful ideas gradually became ordinary features of modern languages such as Kotlin, Swift, Scala and many others.'
tags = []
posted = 'June 22, 2026'
author = 'Aliaksei Ivanouski'
+++

I have worked with Android for more than a decade, so I'm most experienced with Kotlin and Java.
In my free time, I learn other languages: Rust, Swift, Scala, and Clojure,
as well as the Functional Programming (FP) paradigm in general.
The observation became clear to me at some point.
Modern languages increasingly combine both approaches. Here is why I think so.

### 1. Jetpack Compose

Jetpack Compose is one of the clearest examples of functional influence in Android development.
Its core is a function that renders the UI. Besides the functions that render the UI,
there are Effects that perform actions or, in other words, manage Side Effects.
I know someone familiar with FP, and probably with `functional purity`, would say:

> Jetpack Compose functions are not pure functions!

Well, they are not. But Compose encourages us to describe the interface as a function of state:
`UI = render(state)`.
And this idea is very close to how the UI is rendered in the functional world.
The object-oriented world does it in a completely different way, using mutations of objects.

### 2. MVI and Redux

The state-of-the-art pattern for building UI in Android today is MVI (Model-View-Intent).
It was definitely inspired by Redux, and probably by Elm as well. All of them share some similarities.
The `UI` is a function of `state`, and the `state` itself is a function of `oldState` and a user `action`:
`state = reduce(oldState, action)`.
Maybe I'm a little bit biased, but it looks like Elm was the first one on the historical timeline.
For those who don't know, Elm is a functional language.

### 3. Higher-order functions are standard for collections

Operations such as `map`, `filter`, and `reduce` are everywhere now, even in Java and C++.
It's hard to trace the roots of such a popular thing, but those functions
have existed for decades in functional languages.

### 4. Algebraic Data Types (ADT)

Algebraic Data Types look different from language to language and have very strong functional roots.
ADTs consist of two types: Product types and Sum types.
Here is how they are represented in different languages:

| Language | Product type | Sum Type     |
|----------|--------------|--------------|
| Java     | record       | sealed class |
| Kotlin   | data class   | sealed class |
| Scala    | case class   | sealed class |
| Rust     | struct       | enum         |
| Swift    | struct       | enum         |

### 5. Exhaustive pattern matching

Pattern matching works naturally with Algebraic Data Types.
Rust uses `match`, Scala uses `match`, Swift uses `switch`, and Kotlin uses `when`.
The principle stays the same: when all possible cases are matched, the `else` branch is not necessary.
Here is an example in Kotlin:

```kotlin
fun nameIt(type: AlgebraicDataType): String =
    when (type) {
        AlgebraicDataType.Product -> "data class"
        AlgebraicDataType.Sum -> "sealed class"
    }
```

### 6. Errors became explicit values

Functional programming popularized the idea of treating errors as first-class citizens in the app.
It means that, besides normal values, functions should return errors.
Rust uses `Result<Data, Error>`, Scala uses `Either<Error, Data>` (and many other monads/wrappers),
and Kotlin has `Result<Data>`.
Kotlin also deprecated checked exceptions, the OOP way of handling errors, which is the default in Java.

### 7. Modern languages are expression-based instead of statement-based

Here is statement-based code in Kotlin (please don't write it like that):

```kotlin
fun fooName(): String {
    var result

    if (isFoo()) {
        result = "foo"
    } else {
        result = "bar"
    }

    return result
}
```

And the expression-based version:

```kotlin
fun fooName(): String =
    if (isFoo()) "foo" else "bar"
```

We don't need to mutate the variable `result` when working with expressions.
I hope the code looks self-explanatory. Please don't use statements; use expressions instead!

### 8. Reactive programming

Rx libraries brought functional composition into asynchronous and event-driven applications.
Instead of coordinating callbacks or juggling threads manually,
developers represent events as streams and transform them with `map`, `filter`, and other operators.
There are a lot of libraries similar to Rx, and they all have their roots in Functional Reactive Programming.

## Conclusion

Functional programming did not need to replace object-oriented programming.
The most successful functional ideas no longer look unusual - they simply look like modern programming.
Functional programming goes together with object-oriented programming,
and we can say that it is beneficial to both.