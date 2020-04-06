
start =0
_end=1
index =0


function load(value,option)
    start = tonumber(option["start"])
    _end =tonumber(option["end"])
    index=start
end

function next()
    temp = index
    if index < _end then 
        index=index+1
        return tostring(temp), false
    else
        return tostring(temp), true
    end
end

function reset()
    index=start
end