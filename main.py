from collections import defaultdict
import os
import tkinter.filedialog
import tkinter as tk
import re
revar = re.compile("/[a-z]/")


def parse_files(filenames):
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


def parse_user_key(userkeytxt):
	userkey = revar.split(userkeytxt)
	uservars = revar.findall(userkeytxt)
	uservars = tuple(ord(var[1:-1])-97 for var in uservars)
	return userkey, uservars


def keystr(key):
	res = ""
	for i in range(len(key)-1):
		res += key[i] + f"/{chr(i+97)}/"
	res += key[-1]
	return res


def ask_user(parsed):
	def show_entry_fields():
		for k in range(len(keyorder)):
			userinput = entries[k].get().strip()
			if userinput:
				keyuser, valuser = parse_user_key(userinput)
				keysuser[keyorder[k]] = keyuser
				valsuser[keyorder[k]] = valuser
		master.quit()

	def cancel():
		master.quit()
		raise SystemExit()

	keysuser = {}
	valsuser = {}
	master = tk.Tk()
	master.title("Simple Renamer")
	entries = []
	keyorder = []
	for i, key in enumerate(parsed):
		keyorder.append(key)
		keytxt = keystr(key)
		tk.Label(master, text=keytxt, width=len(keytxt)).grid(row=i*2)
		entries.append(tk.Entry(master, width=len(keytxt)))
		entries[-1].grid(row=i*2+1)
		entries[-1].insert(tk.END, keytxt)

	tk.Button(master, text='Cancel', command=cancel).grid(row=3, column=0, sticky=tk.W, pady=4)
	tk.Button(master, text='Apply', command=show_entry_fields).grid(row=3, column=1, sticky=tk.W, pady=4)
	tk.mainloop()

	return keysuser, valsuser


def build_name(key, val):
	name = key[0]
	for i in range(1, len(key)):
		name += val[i-1]
		name += key[i]
	return name


def global_rename(path, parsed, keysuser, valsuser):
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
	root = tk.Tk()
	root.withdraw()
	# _path = tk.filedialog.askdirectory()
	_path = os.getcwd()
	(_, _, _files) = next(os.walk(_path))
	_parsed = parse_files(_files)
	if _parsed:
		try:
			_keysuser, _valsuser = ask_user(_parsed)
			global_rename(_path, _parsed, _keysuser, _valsuser)
		except SystemExit:
			# allows us to exit gracefully
			pass
