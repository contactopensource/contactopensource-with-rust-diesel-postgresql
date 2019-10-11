# Contact Open Source with Rust Diesel PostgreSQL

Contact Open Source is free contact management software, akin to a
contacts app, or address book, or customer relationship manganer (CRM).


## Introduction

The goal of this project is to provide free open source software,
that is especially easy to change and adapt for new kinds of uses,
in collaborationwith software programmers and application developers.


## Ideas

Ideas for contact card formats:

  * [VCard using structured text](https://wikipedia.org/wiki/VCard)

  * [HCard using HTML](https://wikipedia.org/wiki/HCard) (and XCard using XML and similar format)

  * [JCard using JSON and RFC 7095](https://tools.ietf.org/html/rfc7095) (and YCard using YAML and simlar format)

Ideas for item associations:

  * Feed: such as a feed of news, weather, stories, posts, etc.

  * Note: such as a memo, advisory, update, additional information, etc.

  * Résumé: such as summary of work experience, skills, capabilties, etc.


## Implementation

This implementation uses the Rust programming language, Diesel crate which is an ORM, and PostgreSQL database.

We welcome contributing code that provides additional implementations, such as for more programming languages (e.g. Java, Go, C) and databases (e.g. MySQL, SQL Server, Oracle).

The current code is deliberately very simple: there is only one table: a contact item.

We encourage you to extend this code as you like, for your own kinds of SQL and functionality.
