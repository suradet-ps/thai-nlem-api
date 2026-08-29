# Thai NLEM API

```
████████╗██╗  ██╗ █████╗ ██╗███╗   ██╗██╗     ███████╗███╗   ███╗
╚══██╔══╝██║  ██║██╔══██╗██║████╗  ██║██║     ██╔════╝████╗ ████║
   ██║   ███████║███████║██║██╔██╗ ██║██║     █████╗  ██╔████╔██║
   ██║   ██║  ██║██╔══██║██║██║╚██╗██║██║     ██╔══╝  ██║╚██╔╝██║
   ██║   ██║  ██║██║  ██║██║██║ ╚████║███████╗███████╗██║ ╚═╝ ██║
   ╚═╝╚═╝  ╚═╝╚═╝  ╚═╝╚═╝  ╚═══╝╚══════╝╚══════╝╚═╝     ╚═╝
```

---

## ◆ PULSE

Thailand's essential medicines list is the country's promise about
what healthcare must never run out of - and it has been locked in
spreadsheets. This is the unofficial REST API that opens it:
paracetamol to anti-TB drugs, searchable by generic or synonym name,
returned as clean JSON with dosage forms, ED level, warnings, and
conditions. Built with Rust, Axum, and SQLx - compile-time-checked
queries, public CORS, and an open door for developers, researchers,
and healthcare tools that need the NLEM programmatically.

| Search ▣ | SQLx-checked ▣ | Public CORS ▣ | Open data ▣ |
|---|---|---|---|

*The pipeline - migrate, seed, serve - is sealed.*

> Built with Rust 1.77 + Axum + SQLx on PostgreSQL - the essential
> medicines list, served like software.
>
> **suradet-ps**, artifact keeper

---

## ◆ IGNITION

One container, four commands.

```
⟫ docker run --name nlem-db -e POSTGRES_USER=myuser -e POSTGRES_PASSWORD=mypassword \
    -e POSTGRES_DB=nlem_api_db -p 5432:5432 -d postgres:16-alpine
⟫ cp .env.example .env
⟫ sqlx database create && sqlx migrate run
⟫ cargo run -- seed
⟫ cargo run
```

The API listens at [http://localhost:3000](http://localhost:3000).

<details>
<summary>Prerequisites</summary>

- [Rust](https://rustup.rs/) toolchain (1.77+)
- PostgreSQL (Docker recommended)
- `sqlx-cli` - `cargo install sqlx-cli`
- The `nlem_2567.csv` source file placed in `data/`

</details>

---

## ◆ ANATOMY

One table, two endpoints, a seeder that owns the truth.

- **Migrates** - `sqlx migrate run` creates `drugs` and
  `drug_categories` with migrations tracked like code - the schema
  has a history, not a memory.
- **Seeds** - `cargo run -- seed` loads `nlem_2567.csv` into
  PostgreSQL - the spreadsheet becomes a database in a few minutes,
  once.
- **Searches** - `GET /api/drugs/search?q=paracetamol` matches
  generic and synonym names case-insensitively - the developer's
  question answered in one query.
- **Answers** - `GET /api/drugs/:id` returns the single record with
  its dosage forms, ED level, recommendations, warnings, and
  conditions - structured JSON, no spreadsheet archaeology.
- **Serves** - Axum answers with public CORS from day one, and the
  `.sqlx` directory keeps offline compilation working without a live
  database.

---

## ◆ RITUALS

**The core ceremony** - the first query:

1. Start the database container; the URL goes into `.env`.
2. Migrate, then seed: the CSV becomes rows, once, carefully.
3. Run the server. `GET /` answers `{"status":"OK"}`.
4. Search: `q=paracetamol` returns the monograph the list holds -
   dosage forms, ED level, warnings, conditions - ready for the
   caller's own format.

**The ceremony of the checked query** - SQLx verifies every statement
against the schema at compile time: a query that does not match the
database never ships. The API's contract with PostgreSQL is enforced
by the compiler.

**The ceremony of the public door** - CORS is open by design: the API
exists to be called by anyone building on Thailand's essential
medicines data - a research tool, a hospital app, a student project.
The door is the point.

---

## ◆ ECHOES

**Where this artifact is heading**

```
migrate ▸ tracked schema, drugs + drug_categories ───────────────────── ▸ sealed
seed    ▸ nlem_2567.csv import ──────────────────────────────────────── ▸ sealed
search  ▸ generic + synonym lookup ──────────────────────────────────── ▸ sealed
serve   ▸ Axum + SQLx, public CORS, offline .sqlx ───────────────────── ▸ sealed
```

**Raising the artifact** - the schema lives in `migrations/`; the
source data in `data/`; the server in `src/`. Contributions follow
the fork-and-PR path with `cargo fmt` applied and tests updated. Open
an issue first to discuss a change.

**Status** - dependencies are maintained through Renovate.

---

```
  ─────────────────────────────────────────
   An essential medicines list kept in a
   spreadsheet is a list waiting to be read.
  ─────────────────────────────────────────
```

Distributed under the [MIT License](LICENSE).