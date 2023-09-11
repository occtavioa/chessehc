use crate::models::{Player, Tournament, Pairing};
use rusqlite::{params, Connection, OpenFlags, Result};
use std::{path::Path, default};

pub fn open_not_create(path: &Path) -> Result<Connection> {
    Connection::open_with_flags(
        path,
        OpenFlags::SQLITE_OPEN_READ_WRITE
            | OpenFlags::SQLITE_OPEN_NO_MUTEX
            | OpenFlags::SQLITE_OPEN_URI,
    )
}

pub fn create_schema(connection: &Connection) -> Result<()> {
    connection.execute_batch(
        "
        CREATE TABLE \"Tournament\" (
            \"Id\"      INTEGER,
            \"Name\"	TEXT NOT NULL,
            \"City\"	TEXT,
            \"FideFederation\"	    TEXT,
            \"DateStart\"	TEXT,
            \"DateEnd\"	TEXT,
            \"TypeTournament\"	TEXT,
            \"Format\"	    TEXT,
            \"ChiefArbiter\"	TEXT,
            \"DeputyChiefArbiter\"	TEXT,
            \"TimeControl\"	TEXT,
            \"NumberRounds\"	INTEGER NOT NULL,
            \"CurrentRound\"	INTEGER,
            PRIMARY KEY(\"Id\")
        );
        CREATE TABLE \"Player\" (
            \"Id\"	INTEGER,
            \"Name\"	TEXT NOT NULL,
            \"Points\"  REAL NOT NULL,
            \"Sex\"	TEXT,
            \"Title\"	TEXT,
            \"Rating\"	INTEGER,
            \"FideFederation\"	TEXT,
            \"FideNumber\"	TEXT,
            \"BirthDate\"	TEXT,
            PRIMARY KEY(\"Id\")
        );
        CREATE TABLE \"Round\" (
            \"Id\"	INTEGER,
            \"Number\"  INTEGER NOT NULL,
            \"Date\"    TEXT,
            PRIMARY KEY(\"Id\")
        );
        CREATE TABLE \"PlayerStateByRound\" (
            \"IdPlayer\"    INTEGER,
            \"IdRound\" INTEGER,
            \"Points\"  REAL
        );
        CREATE TABLE \"MatchByRound\" (
            \"IdRound\" INTEGER NOT NULL,
            \"IdWhite\" INTEGER NOT NULL,
            \"IdBlack\" INTEGER NOT NULL,
            \"PointsWhite\" TEXT,
            \"PointsBlack\" TEXT
        );
        CREATE TABLE \"ByeByRound\" (
            IdRound INTEGER NOT NULL,
            IdPlayer    INTEGER NOT NULL,
            ByePoint    TEXT NOT NULL
        );
        "
    )
}

pub fn insert_tournament(
    tournament: &Tournament,
    connection: &Connection,
) -> Result<usize> {
    connection.execute(
        "
        INSERT INTO \"Tournament\" VALUES
        (
            NULL,
            (?1),
            (?2),
            (?3),
            (?4),
            (?5),
            (?6),
            (?7),
            (?8),
            (?9),
            (?10),
            (?11),
            NULL
        )
        ",
        params![
            &tournament.name,
            &tournament.city,
            &tournament.fide_federation,
            &tournament.date_start,
            &tournament.date_end,
            &tournament.type_tournament,
            &tournament.format,
            &tournament.chief_arbiter,
            &tournament.deputy_chief_arbiter,
            &tournament.time_control,
            &tournament.number_rounds,
        ],
    )
}

pub fn select_tournament(connection: &Connection) -> Result<Tournament> {
    connection.query_row("SELECT * FROM \"Tournament\"", [], |row| {
        Ok(Tournament {
            name: row.get(1)?,
            city: row.get(2)?,
            fide_federation: row.get(3)?,
            date_start: row.get(4)?,
            date_end: row.get(5)?,
            type_tournament: row.get(6)?,
            format: row.get(7)?,
            chief_arbiter: row.get(8)?,
            deputy_chief_arbiter: row.get(9)?,
            time_control: row.get(10)?,
            number_rounds: row.get(11)?,
            current_round: row.get(12)?,
        })
    })
}

pub fn select_players(connection: &Connection) -> Result<Vec<Player>> {
    let mut query = connection.prepare("SELECT * FROM \"Player\"")?;
    let players_iter = query.query_map([], |row| {
        Ok(Player {
            name: row.get(1)?,
            points: row.get(2)?,
            sex: row.get(3)?,
            title: row.get(4)?,
            rating: row.get(5)?,
            fide_federation: row.get(6)?,
            fide_number: row.get(7)?,
            birth_date: row.get(8)?,
        })
    })?.filter(|p| p.is_ok());
    Ok(players_iter
        .map(|player| player.unwrap())
        .into_iter()
        .collect())
}

pub fn insert_player(connection: &Connection, player: &Player) -> Result<usize> {
    connection.execute(
        "
        INSERT INTO \"Player\" VALUES
        (
            NULL,
            (?1),
            0,
            (?2),
            (?3),
            (?4),
            (?5),
            (?6),
            (?7)
        )
        ",
        params![
            &player.name,
            &player.sex,
            &player.title,
            player.rating,
            &player.fide_federation,
            player.fide_number,
            &player.birth_date,
        ],
    )
}

pub fn select_current_round(connection: &Connection) -> Result<Option<u16>> {
    connection.query_row("SELECT CurrentRound FROM \"Tournament\"", [], |row| {
        Ok(row.get(0)?)
    })
}

pub fn select_games(connection: &Connection) -> Result<Vec<Pairing>> {
    Ok(default::Default::default())
}
