import tkinter as tk
import tkinter.font
import tkinter.ttk as ttk
import re
revar = re.compile("/[a-z]/")


def parse_user_key(userkeytxt):
	"""
	Take the userstring and parse the variables to return a key and
	the value indexes.
	:param userkeytxt: A string like "blabla/a/bla/b/.ext"
	:return: The key ("blabla", "bla", ".ext") and the indexes (0, 1)
	"""
	userkey = revar.split(userkeytxt)
	uservars = revar.findall(userkeytxt)
	uservars = tuple(ord(var[1:-1])-97 for var in uservars)
	return userkey, uservars


def keystr(key):
	"""
	Returns a string representing the key, with a letter for each variables
	:param key: A tuple like ("blabla", "bla", ".ext")
	:return: A string like "blabla/a/bla/b/.ext"
	"""
	# Return a string representing a key, with /a/, /b/ etc for variables
	res = ""
	for i in range(len(key)-1):
		res += key[i] + f"/{chr(i+97)}/"
	res += key[-1]
	return res


class App:
	entries: list
	keyorder: list
	keysuser: dict
	valsuser: dict

	def __init__(self, root, parsed):
		self.root = root
		self.build(parsed)

	def build(self, parsed):
		font = tk.font.Font(family="Lucida Grande", size=12)
		# instanciate the GUI
		frame = ttk.Frame(self.root)
		frame.pack()
		self.root.title("Simple Renamer")
		# the list of text Entry
		self.entries = []
		# the list of template keys
		# (used to remember which entries correspond to which key, since a dictionnary is unordered)
		self.keyorder = []
		# label telling the templates found
		ttk.Label(
			frame,
			text=f"Found {len(parsed)} template{'s' if len(parsed) > 1 else ''}:",
			font=font
		).grid(row=0)
		# for each template key i
		for i, key in enumerate(parsed):
			# append it to the list of keys
			self.keyorder.append(key)
			# get the string representation of the template key
			keytxt = keystr(key)
			# compute a widget width (proportionnal to key length)
			width = max(int(len(keytxt)*1.2+20), 30)
			# add a label with the key str and the number of affected files
			ttk.Label(frame, text=keytxt+f"  ({len(parsed[key])} files)", width=width, font=font).grid(
				row=(i+1)*2, pady=(10, 0)
			)
			# add the corresponding entry
			self.entries.append(ttk.Entry(frame, width=width, font=font))
			self.entries[-1].grid(row=(i+1)*2+1)
			self.entries[-1].insert(tk.END, keytxt)

		# bind ENTER key to APPLY
		self.root.bind('<Return>', self.retrieve_user_input)
		# Add a Cancel button (same effect as closing the window)
		ttk.Button(frame, text='Cancel', command=self.cancel).grid(
			row=(len(parsed)+1)*2, column=0, sticky=tk.W, pady=4
		)
		# Add an APPLY button
		ttk.Button(frame, text='Apply', command=self.retrieve_user_input).grid(
			row=(len(parsed)+1)*2, column=0, sticky=tk.E, pady=4
		)
		# bind the cancel function to windows closing event
		self.root.protocol("WM_DELETE_WINDOW", self.cancel)

	def retrieve_user_input(self, _=None):
		# On APPLY, read and parse what the user wrote in the text Entries
		self.keysuser = {}
		self.valsuser = {}
		# for each index in the list of keys
		for k in range(len(self.keyorder)):
			# get the text from the entry k
			userinput = self.entries[k].get().strip()
			# if there is any text
			if userinput:
				# parse it
				keyuser, valuser = parse_user_key(userinput)
				# map it with the corresponding key
				self.keysuser[self.keyorder[k]] = keyuser
				self.valsuser[self.keyorder[k]] = valuser
		self.root.quit()

	def cancel(self):
		# On CANCEL, close the GUI and exit the program
		self.root.quit()
		raise SystemExit()
