
start =0
_end=0
index =0
array = {}
function trim(s)
    return (s:gsub("^%s*(.-)%s*$", "%1"))
end
function read_dir(dir)
    local iter, dir_obj=lfs.dir(dir)
    for path in iter, dir_obj do
        local full_path=dir..'/'..path
        if path == '.' or path =='..' then
           goto continue
        end
        local attr=lfs.attributes(full_path)
        if attr.mode =='directory' then
            read_dir(full_path)
        elseif attr.mode =='file' then
            array[index]=full_path
            index=index+1
        end
        ::continue::
    end
    dir_obj:close()
end
function load(value,option)
    local path= trim(value)
    base_dir=trim(base_dir)
    base_path=lpath.join(base_dir,path)
    read_dir(base_path)
    _end=index
    index=0
end

function next()
    if index < _end then 
        local path=array[index]
        local short = lpath.relative(base_path, path)
        index=index+1
        return path,short, false
    else
        return '', '',true
    end
end


function reset()
    index=start
end