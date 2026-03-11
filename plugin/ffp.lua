vim.api.nvim_create_user_command("FFP", function()
	require("ffp").open()
end, {})
