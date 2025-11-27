local debug = {}

function debug.printable(table, spacing)
    for key, value in pairs(table) do
        if type(value) == "table" then
            print(spacing..key..":")
            debug.printable(value, spacing.."|\t")
        else
            print(spacing..key..": "..value)
        end
    end
end

return debug