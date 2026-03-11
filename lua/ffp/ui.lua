local M = {}

function M.open()
	local buf = vim.api.nvim_create_buf(false, true)
	local width = math.floor(vim.o.columns * 0.8)
	local height = math.floor(vim.o.lines * 0.8)
	local win = vim.api.nvim_open_win(buf, true, {
		relative = "editor",
		row = 2,
		col = 4,
		width = width,
		height = height,
		border = "rounded",
	})

	local results = {}

	local backend = "/Users/raj/dev/ffp.nvim/rust/backend/target/release/backend"
	vim.fn.jobstart(backend, {
		stdout_buffered = true,

		on_stdout = function(_, data)
			for _, line in ipairs(data) do
				if line ~= "" then
					table.insert(results, line)
				end
			end

			vim.api.nvim_buf_set_lines(buf, 0, -1, false, results)
		end,
	})
end

return M
