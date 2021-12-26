
from . import Widget

class Field(Widget):
	
	
	def __init__(self, char_size=1):
		self.width = 0
		self.height = 0
		self.char_size = char_size
		self.pad = None
		self.center = (0, 0)
		self.redraw = False
		self.offset = (0, 0)
	
	def set_backend(self, backend):
		self.backend = backend
		self.pad = self.backend.create_pad(self.width * self.char_size, self.height)
	
	def set_char_size(self, char_size):
		self.char_size = char_size
		self.pad = self.backend.create_pad(self.width * self.char_size, self.height)
	
	def set_dimensions(self, offset, width, height, keep=False):
		self.pad.resize(width * self.char_size, height)
		self.offset = offset
		self.width = width
		self.height = height
		self.redraw = True
		self.change()
	
	def change_cell(self, x, y, char, style=None):
		x -= self.offset[0]
		y -= self.offset[1]
		if x < 0 or y < 0 or x >= self.width or y >= self.height:
			return
		self.pad.write(x * self.char_size, y, char, style)
		self.change()
	
	def draw_all(self, values, mapping):
		# This code is hot. Performance gains can be worth the price of code quality
		brushes = [(char, self.pad.get_raw_style(style)) for (char, style) in mapping]
		for x in range(0, self.width):
			sized_x = x * self.char_size
			for y in range(0, self.height):
				brush = brushes[values[x+self.width*y]]
				self.pad.set_char(sized_x, y, brush[0], brush[1])
		self.change()
	
	def set_center(self, x, y):
		self.center = (x, y)
		self.change()
	
	def _round_width(self, x):
		return x // self.char_size * self.char_size
	
	def draw(self, target):
		center_x = self.center[0] - self.offset[0]
		center_y = self.center[1] - self.offset[1]
		target.draw_pad(
			self.pad,
			src_x = max(0, min(
				self._round_width(self.pad.width - target.width),
				self._round_width(center_x * self.char_size - target.width // 2)
			)),
			src_y = max(0, min(self.pad.height - target.height, center_y - target.height // 2)),
			width = self._round_width(target.width),
			dest_x = max(0, (target.width - self.pad.width) // 2),
			dest_y = max(0, (target.height - self.pad.height) // 2)
		)
	
	@classmethod
	def from_xml(cls, children, attr, text):
		return cls(char_size=int(attr.get("char-size", 1)))
