for D in *; do
	if [ -d "${D}" ]; then
		( cd "${D}" && cargo clean && ./build.sh )
	fi
done

