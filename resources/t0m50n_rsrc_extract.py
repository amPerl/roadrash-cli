from struct import unpack
import os

OUTPUT_PATH = 'output'

file_handle = open('CITY.CRS', 'rb')

file_signature = file_handle.read(4)
file_signature = file_signature

file_handle.read(12)

file_list_offset = file_handle.read(4)
file_list_offset = unpack('<I', file_list_offset)[0]

data_offset = file_handle.read(4)
data_offset = unpack('<I', data_offset)[0]

file_handle.read(file_list_offset - 8)

no_of_files = data_offset // 32

files = [None] * no_of_files
for i in range(no_of_files):
    file_type = file_handle.read(4)
    file_type = file_type.decode('ascii')[::-1]

    file_index = file_handle.read(4)
    file_index = unpack('<I', file_index)[0]

    file_offset = file_handle.read(4)
    file_offset = unpack('<I', file_offset)[0]

    file_size = file_handle.read(4)
    file_size = unpack('<I', file_size)[0]

    file_handle.read(16)

    prev_seek = file_handle.tell()
    file_handle.seek(file_offset)
    file_data = file_handle.read(file_size)
    file_handle.seek(prev_seek)

    files[i] = dict(
        file_type=file_type,
        file_index=file_index,
        file_offset=file_offset,
        file_size=file_size,
        file_data=file_data
    )

file_handle.close()

for f in files:
    filepath = '{}_{}.raw'.format(f['file_type'], f['file_index'])
    filepath = os.path.join(OUTPUT_PATH, filepath)
    with open(filepath, 'wb') as fh:
        fh.write(f['file_data'])