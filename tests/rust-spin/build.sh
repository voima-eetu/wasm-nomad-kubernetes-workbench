for D in *; do
	if [ -d "${D}" ]; then
		( cd "${D}" && ./build.sh )
	fi
done

