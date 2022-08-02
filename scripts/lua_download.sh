#/bin/bash
ver=$1
dir=$2
curl -X GET  https://www.lua.org/ftp/lua-${ver}.tar.gz --output ${dir}/lua-${ver}.tar.gz
(cd ${dir} ; tar -xf lua-${ver}.tar.gz)
(cd ${dir} ; mv lua-${ver}/* . ; rm -rf lua-${ver})
