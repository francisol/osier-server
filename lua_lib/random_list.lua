start =1
_end=1
index =1
function split(s, p)
    local rt= {}
    string.gsub(s, '[^'..p..']+', function(w) table.insert(rt, w) end )
    return rt
end

function load(value,option)
    list = split(value,",")
    start=1
    _end = #list +1
    index=start
end

function next()
    temp = list[index]
    if index < _end then 
        index=index+1
        return temp,temp, false
    else
        return '','', true
    end
end

function reset()
    index=start
end
