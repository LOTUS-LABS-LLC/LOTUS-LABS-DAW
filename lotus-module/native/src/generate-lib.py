import os
import sys

funclist = []

if len(sys.argv) <= 1:
	libname = str(input("enter name of rs file to import ").replace(".rs",""))
else:
	libname =sys.argv[1].replace(".rs","")

try:
	os.remove("lib.rs")
except:
	print("no lib.rs to delete.")

isize = sum(1 for l in open(str(libname+".rs"), "r"))

mod = open(str(libname+".rs"), "r")

lib = open("lib.rs", "w+")

jsm = open("../../lib/index.js", "w+")

#write preliminary headers for rust file
lib.write("use neon::prelude::*;\nuse neon::register_module;\nmod "+libname+";\n")

#now for the js module
jsm.write("var mod = require('../native');\nclass OutFn {\nconstructor(){\n}\n")

print("iterating",isize,"lines...")

for ln in range(0,isize):
	target_line = False;
	ic = 0
	line = mod.readline()
	for c in line:
		if line[ic] == ' ':
			pass
		if str(line[ic:(ic+7)]) == "pub fn ":
			target_line = True;
			print("FOUND["+line[0:-1]+"]")
		if target_line == True:
			#find name...
			stripped = line[ic+7:].replace(" ", "").replace("\n","")
			endex = 0
			end = 0
			for p in stripped:
				if p == "(":
					end = endex
				endex+=1
			#kick extension...
			#append to function namelist
			funclist.append(stripped[:end])
			#write neon binding function to lib.rs
			lib.write("fn "+stripped[:end]+"_fn(mut cx: FunctionContext) -> JsResult<JsUndefined> {\n"+libname+"::"+stripped[:end]+"();\nOk(cx.undefined())\n}\n")
			#connect function to module exporting class
			jsm.write(stripped[:end]+" = mod."+stripped[:end]+"_fn;\n")
			target_line = False	
		ic+=1

#finished js module
jsm.write("}\nmodule.exports = new OutFn();")

#register module and all functions
lib.write("register_module!(mut cx, {\n")

for f in funclist:
	lib.write("cx.export_function(\""+f+"_fn\", "+f+"_fn)?;\n")
	
lib.write("Ok(())\n});\n")




