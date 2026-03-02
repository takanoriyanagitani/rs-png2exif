iwasi=./rs-png2exif.wasm

wasm-opt \
	-Oz \
	-o opt.wasm \
	--enable-bulk-memory \
	--enable-nontrapping-float-to-int \
    --enable-simd \
	"${iwasi}"
