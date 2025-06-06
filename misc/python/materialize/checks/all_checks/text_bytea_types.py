# Copyright Materialize, Inc. and contributors. All rights reserved.
#
# Use of this software is governed by the Business Source License
# included in the LICENSE file at the root of this repository.
#
# As of the Change Date specified in that file, in accordance with
# the Business Source License, use of this software will be governed
# by the Apache License, Version 2.0.
from textwrap import dedent

from materialize.checks.actions import Testdrive
from materialize.checks.checks import Check


class TextByteaTypes(Check):
    def initialize(self) -> Testdrive:
        return Testdrive(
            dedent(
                """
            > CREATE TABLE text_bytea_types_table (text_col TEXT, bytea_col BYTEA);
            > INSERT INTO text_bytea_types_table VALUES ('aaaa', '\\xAAAA'), ('това е', 'текст');
        """
            )
        )

    def manipulate(self) -> list[Testdrive]:
        return [
            Testdrive(dedent(s))
            for s in [
                """
                > CREATE MATERIALIZED VIEW string_bytea_types_view1 AS
                  SELECT text_col, bytea_col, 'това'::TEXT, '\\xAAAA'::BYTEA
                  FROM text_bytea_types_table
                  WHERE text_col >= ''::TEXT AND bytea_col >= ''::BYTEA;

                > INSERT INTO text_bytea_types_table SELECT DISTINCT text_col, bytea_col FROM text_bytea_types_table;
                """,
                """
                > CREATE VIEW string_bytea_types_view2 AS
                  SELECT text_col, bytea_col, 'това'::TEXT, '\\xAAAA'::BYTEA
                  FROM text_bytea_types_table
                  WHERE text_col >= ''::TEXT AND bytea_col >= ''::BYTEA;

                > INSERT INTO text_bytea_types_table SELECT DISTINCT text_col, bytea_col FROM text_bytea_types_table;
                """,
            ]
        ]

    def validate(self) -> Testdrive:
        return Testdrive(
            dedent(
                """
                > SELECT text_col, text, LENGTH(bytea_col), LENGTH(bytea) FROM string_bytea_types_view1;
                aaaa това 2 2
                aaaa това 2 2
                aaaa това 2 2
                "това е" това 10 2
                "това е" това 10 2
                "това е" това 10 2

                > SELECT text_col, text, LENGTH(bytea_col), LENGTH(bytea) FROM string_bytea_types_view2;
                aaaa това 2 2
                aaaa това 2 2
                aaaa това 2 2
                "това е" това 10 2
                "това е" това 10 2
                "това е" това 10 2
            """
            )
        )
