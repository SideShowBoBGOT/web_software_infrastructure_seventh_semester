from typing import Any
from marshmallow import Schema
from typing import cast


def cast_to_schema(schema: Schema, json_data: Any) -> Any:
    return cast(Any, schema.load(json_data))