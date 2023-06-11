return {
    _DICT = typst.from_table({ world = "World!" }),
	_TEXT = typst.from_text("World!"),
	_JSON = typst.from_json([[
		{
			"message": "Hello, world!"
		}
	]])
}

