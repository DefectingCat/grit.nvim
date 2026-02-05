-- Test script to load and test the Grit plugin

-- Add current directory to runtimepath and package.cpath
local test_dir = vim.fn.expand('<sfile>:p:h')
local plugin_dir = test_dir:gsub("/tests$", "")  -- 向上找到项目根目录
vim.opt.rtp:prepend(plugin_dir)

-- Add the plugin's lua directory to package.cpath
package.cpath = package.cpath .. ';' .. plugin_dir .. '/lua/?.so'

-- Function to handle test failures
local function test_failure(msg)
  vim.api.nvim_err_writeln(string.format("TEST FAILED: %s", msg))
  -- Use os.exit() to ensure Neovim exits with non-zero code
  vim.schedule(function()
    vim.cmd("cq! 1")  -- Exit with code 1 (failure)
  end)
  error(msg)  -- This will stop further execution
end

-- Load the plugin directly (bypassing the lua/grit/init.lua which causes loops)
local ok, grit = pcall(require, 'grit')
if not ok then
  test_failure(string.format("Error loading Grit plugin: %s", grit))
end

-- Test 1: Check if the plugin returned a table
if type(grit) ~= 'table' then
  test_failure("Plugin did not return a table")
end

-- Test 2: Check if 'hello' key exists in the returned table
if grit.hello ~= 'world' then
  test_failure(string.format("Expected grit.hello to be 'world', got '%s'", grit.hello))
end

-- Test 3: Check if 'grit' key exists in the returned table
if grit.grit ~= 'initialized' then
  test_failure(string.format("Expected grit.grit to be 'initialized', got '%s'", grit.grit))
end

-- Test 4: Check if GritHello command is available
local commands = vim.api.nvim_get_commands({})
if not commands['GritHello'] then
  test_failure("GritHello command is not registered")
end

-- Test 5: Check if Grit command is available
if not commands['Grit'] then
  test_failure("Grit command is not registered")
end

-- Test 6: Test executing GritHello command
local ok2, err2 = pcall(function()
  vim.cmd("GritHello")
end)
if not ok2 then
  test_failure(string.format("Error executing GritHello command: %s", err2))
end

-- Summary of tests passed
print("All tests passed!")
print("- Plugin loaded successfully")
print("- Plugin returns a valid table")
print("- grit.hello == 'world'")
print("- grit.grit == 'initialized'")
print("- GritHello command is registered")
print("- Grit command is registered")
print("- GritHello command executes successfully")

-- Exit Neovim with success code
vim.schedule(function()
  vim.cmd("qall")
end)
