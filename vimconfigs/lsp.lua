local function load_config(plugin_dir)
  local config_path = plugin_dir .. '/vimconfigs/config.json'
  local default_config = {
    autocomplete_enabled = true,
    auto_trigger = true,
    trigger_mode = 'both',
    map_tab_complete = false
  }
  
  local f = io.open(config_path, "r")
  if not f then
    return default_config
  end
  
  local content = f:read("*all")
  f:close()
  
  local ok, parsed = pcall(vim.fn.json_decode, content)
  if not ok then
    return default_config
  end
  
  for k, v in pairs(default_config) do
    if parsed[k] == nil then
      parsed[k] = v
    end
  end
  
  return parsed
end

local function setup_c4_buffer(bufnr, plugin_dir)
  local config = load_config(plugin_dir)
  local lsp_bin = plugin_dir .. '/bin/c4-lsp'
  
  vim.lsp.start({
    name = 'c4-lsp',
    cmd = { lsp_bin },
    root_dir = vim.fs.dirname(vim.fs.find({ 'c4.peg' }, { upward = true })[1] or plugin_dir),
    settings = {},
    on_attach = function(client, bufnr)
      local opts = { buffer = bufnr, silent = true }
      vim.keymap.set('n', 'K', vim.lsp.buf.hover, opts)
      
      if config.autocomplete_enabled then
        vim.bo[bufnr].omnifunc = 'v:lua.vim.lsp.omnifunc'
        vim.keymap.set('i', '<C-Space>', '<C-x><C-o>', opts)
        
        if config.map_tab_complete then
          vim.keymap.set('i', '<Tab>', function()
            return vim.fn.pumvisible() == 1 and '<C-n>' or '<Tab>'
          end, { buffer = bufnr, expr = true })
          
          vim.keymap.set('i', '<S-Tab>', function()
            return vim.fn.pumvisible() == 1 and '<C-p>' or '<S-Tab>'
          end, { buffer = bufnr, expr = true })
          
          vim.keymap.set('i', '<CR>', function()
            return vim.fn.pumvisible() == 1 and '<C-y>' or '<CR>'
          end, { buffer = bufnr, expr = true })
        end

        if config.auto_trigger then
          vim.api.nvim_create_autocmd('TextChangedI', {
            buffer = bufnr,
            callback = function()
              if vim.fn.pumvisible() == 0 then
                local col = vim.fn.col('.')
                local line = vim.fn.getline('.')
                local before = line:sub(1, col - 1)
                
                local should_trigger = false
                if config.trigger_mode == "methods" then
                  if before:match('%.[%w_]*$') or before:match('%->[%w_]*$') then
                    should_trigger = true
                  end
                elseif config.trigger_mode == "words" then
                  if before:match('[%a_@]$') then
                    should_trigger = true
                  end
                else
                  if before:match('[%a_@.]$') or before:match('%->$') then
                    should_trigger = true
                  end
                end
                
                if should_trigger then
                  vim.api.nvim_feedkeys(
                    vim.api.nvim_replace_termcodes('<C-x><C-o>', true, true, true),
                    'n',
                    false
                  )
                end
              end
            end
          })
        end
      end
    end
  })
end

function RegisterC4Lsp(plugin_dir)
  vim.api.nvim_create_autocmd('FileType', {
    pattern = 'c4',
    callback = function(args)
      setup_c4_buffer(args.buf, plugin_dir)
    end
  })
  
  if vim.bo.filetype == 'c4' then
    setup_c4_buffer(vim.api.nvim_get_current_buf(), plugin_dir)
  end
end
