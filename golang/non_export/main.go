package main

import "non_export/non_export"

func main() {
	var a struct {
		Foo int
	}
	var b struct {
		Foo  int
		hoge int
	}
	var c interface {
		Foo()
		hoge()
	}
	var d interface {
		Foo()
	}
	var e interface{}
	_ = a == non_export.NonExportStruct
	_ = b == non_export.NonExportStruct
	_ = c == non_export.NonExportInterface
	_ = d == non_export.NonExportInterface
	_ = e == non_export.NonExportInterface
}
