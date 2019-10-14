from collections import defaultdict
import os
import sys
import tkinter.filedialog
import tkinter as tk
import tkinter.ttk as ttk
import re
from gui import revar, App


def parse_files(filenames):
	"""
	Given a list of filenames, build the template dict
	:param filenames: A list of filename
	:return: The template dict <key (,), values [(,), ...]>
	"""
	# build the <template, [vals]> mapping
	reg = re.compile(r"\d+(?:\.\d+)?")
	parsed = defaultdict(list)
	for filename in filenames:
		file, ext = tuple(filename.rsplit(".", 1))
		key = reg.split(file)
		key[-1] += "."+ext
		parsed[tuple(key)].append(tuple(reg.findall(file)))

	# eliminate keys with less than 3 vals
	todelete = []
	for key, vals in parsed.items():
		if len(vals) <= 3:
			todelete.append(key)
	for key in todelete:
		del parsed[key]

	# make constants vals part of the template (ex: 1080 in "1080p")
	newkeys = {}
	newvals = {}
	# for each key
	for key in parsed:
		# build a list indicating which values are constant
		cvals = []
		valsize = len(parsed[key][0])
		# for each index in a value
		for i in range(valsize):
			ival = parsed[key][0][i]
			# if all the values of 1 index are the same
			if all(val[i] == ival for val in parsed[key]):
				# append this values to the constants
				cvals.append(ival)
			else:
				# else, append "/a/" to mark a variable
				cvals.append("/a/")
		# if there is any constants
		if any(cval != "/a/" for cval in cvals):
			# newkey = rebuild the key with cvals, and split it again on the /a/
			newkey = tuple(revar.split(build_name(key, cvals)))
			# if the new key is different from the old key
			if newkey != key:
				newkeys[key] = newkey
				# newvals = rebuild the list of vals with only variable values in it
				newvals[key] = tuple([tuple(val[i] for i in range(valsize) if cvals[i] == "/a/") for val in parsed[key]])
	# for each old keys that have been changed
	for key in newkeys:
		# set the new vals for the new key
		parsed[newkeys[key]] = newvals[key]
		# delete the old key
		del parsed[key]
	return parsed


def build_name(key, val):
	"""
	Use a key and a val to rebuild a filename
	:param key: Tuple like ("blabla", "bla", ".ext")
	:param val: Tuple like ("05", "12")
	:return: filename as string like "blabla05bla12.ext"
	"""
	name = key[0]
	for i in range(1, len(key)):
		name += val[i-1]
		name += key[i]
	return name


def global_rename(path, parsed, keysuser, valsuser):
	"""
	Given the path, the template dict, the user keys and user val indexes,
	rename the files.
	:param path: Path of the files
	:param parsed: Template dict <key (,), values [(,), ...]>
	:param keysuser: Mapping old key->user key
	:param valsuser: Mapping old key->value indexes
	"""
	# build the file name mapping
	newfilenames = {}
	for key in parsed:
		for val in parsed[key]:
			newval = tuple(val[v_i] for v_i in valsuser[key])
			oldname = build_name(key, val)
			newname = build_name(keysuser[key], newval)
			newfilenames[oldname] = newname

	# rename the files
	filenames = os.listdir(path)
	for filename in filenames:
		if filename in newfilenames:
			os.rename(
				os.path.join(path, filename),
				os.path.join(path, newfilenames[filename])
			)


if __name__ == '__main__':
	# NOTE: build with pyinstaller --noconsole main.py
	try:
		if getattr(sys, 'frozen', False):
			# .EXE
			_path = os.getcwd()
		else:
			# DEBUG
			_root = tk.Tk()
			_root.withdraw()
			_path = tk.filedialog.askdirectory()
			if _path == ():
				raise SystemExit()
			_root.destroy()
		(_, _, _files) = next(os.walk(_path))
		_parsed = parse_files(_files)
		if _parsed:
			_root = tk.Tk()
			_root.style = ttk.Style()
			# ('clam', 'alt', 'default', 'classic')
			_root.style.theme_use("clam")
			_app = App(_root, _parsed)
			_root.mainloop()
			global_rename(_path, _parsed, _app.keysuser, _app.valsuser)
	except SystemExit:
		# allows us to exit gracefully
		pass
