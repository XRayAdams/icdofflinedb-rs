/*
 * Copyright (c) 2025 Konstantin Adamov
 *  SPDX-License-Identifier: MIT
 *
 *  For full license text, see the LICENSE file in the repo root.
 */

use rusqlite::{Connection, Result, OpenFlags};
use std::path::Path;

#[derive(Debug, Clone)]
pub struct IcdCode {
    pub cpk_id: i64,
    pub type_id: i64,
    pub code: String,
    pub description: String,
    pub icd_type: String,
}

#[derive(Debug, Clone)]
pub struct CodeSection {
    pub id: i64,
    pub from_code: String,
    pub to_code: String,
    pub description: String,
}

#[derive(Debug)]
pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let conn = Connection::open_with_flags(path, OpenFlags::SQLITE_OPEN_READ_ONLY)?;
        Ok(Self { conn })
    }

    pub fn search(
        &self,
        query: &str,
        icd9_diag: bool,
        icd9_proc: bool,
        icd10_diag: bool,
        icd10_proc: bool,
        selected_section: Option<&CodeSection>,
    ) -> Result<Vec<IcdCode>> {
        let icd9_diag_int = if icd9_diag { 1 } else { 0 };
        let icd9_proc_int = if icd9_proc { 1 } else { 0 };
        let icd10_diag_int = if icd10_diag { 1 } else { 0 };
        let icd10_proc_int = if icd10_proc { 1 } else { 0 };

        let search_pattern = format!("%{}%", query);
        let section_id = selected_section.map(|s| s.id).unwrap_or(0);
        let (from_code, to_code) = if let Some(section) = selected_section {
            (section.from_code.as_str(), section.to_code.as_str())
        } else {
            ("", "")
        };

        let mut stmt = self.conn.prepare(
            "SELECT CPK_ICDMASTERLIST, CFK_ICDTYPE, CODE, DESCRIPTION, TYPE 
             FROM JoinedMaster 
             WHERE 
             (
                (?1 = 1 AND CFK_ICDTYPE = 2) OR
                (?2 = 1 AND CFK_ICDTYPE = 3) OR
                (?3 = 1 AND CFK_ICDTYPE = 0) OR
                (?4 = 1 AND CFK_ICDTYPE = 1)
             )
             AND
             (description LIKE ?5 OR CODE LIKE ?5)
             AND
             (
                ?6 = 0 OR (substr(CODE, 1, 3) >= ?7 AND substr(CODE, 1, 3) <= ?8)
             )
            LIMIT 500"
        )?;

        let code_iter = stmt.query_map(
            rusqlite::params![
                icd9_diag_int,
                icd9_proc_int,
                icd10_diag_int,
                icd10_proc_int,
                search_pattern,
                section_id,
                from_code,
                to_code
            ],
            |row| {
                Ok(IcdCode {
                    cpk_id: row.get(0)?,
                    type_id: row.get(1)?,
                    code: row.get(2)?,
                    description: row.get(3)?,
                    icd_type: row.get(4)?,
                })
            },
        )?;

        let mut codes = Vec::new();
        for code in code_iter {
            codes.push(code?);
        }
        Ok(codes)
    }

    pub fn get_sections(&self) -> Result<Vec<CodeSection>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, FromCode, ToCode, Description FROM CodeSections ORDER BY Description"
        )?;

        let section_iter = stmt.query_map([], |row| {
            Ok(CodeSection {
                id: row.get(0)?,
                from_code: row.get(1)?,
                to_code: row.get(2)?,
                description: row.get(3)?,
            })
        })?;

        let mut sections = Vec::new();
        for section in section_iter {
            sections.push(section?);
        }
        Ok(sections)
    }

    pub fn get_master_record(&self, id: i64) -> Result<Option<IcdCode>> {
        let mut stmt = self.conn.prepare(
            "SELECT CPK_ICDMASTERLIST, CFK_ICDTYPE, CODE, DESCRIPTION, TYPE 
             FROM JoinedMaster 
             WHERE CPK_ICDMASTERLIST = ?"
        )?;

        let mut rows = stmt.query([id])?;

        if let Some(row) = rows.next()? {
            Ok(Some(IcdCode {
                cpk_id: row.get(0)?,
                type_id: row.get(1)?,
                code: row.get(2)?,
                description: row.get(3)?,
                icd_type: row.get(4)?,
            }))
        } else {
            Ok(None)
        }
    }

    pub fn find_proper_id(&self, is_icd9: bool, id: i64) -> Result<i64> {
        let query_statement = if is_icd9 {
            format!("SELECT ICD10 FROM CrossWalk WHERE ICD9 = {}", id)
        } else {
            format!("SELECT ICD9 FROM CrossWalk WHERE ICD10 = {}", id)
        };

        let mut stmt = self.conn.prepare(&query_statement)?;
        let mut rows = stmt.query([])?;

        if let Some(row) = rows.next()? {
            Ok(row.get(0)?)
        } else {
            Ok(0)
        }
    }
}
