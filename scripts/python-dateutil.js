var Module = Module
Module.checkABI(1);
if (!Module.expectedDataFileDownloads) {
    Module.expectedDataFileDownloads = 0;
    Module.finishedDataFileDownloads = 0
}
Module.expectedDataFileDownloads++;
(function() {
    var loadPackage = function(metadata) {
        var PACKAGE_PATH;
        if (typeof window === "object") {
            PACKAGE_PATH = window["encodeURIComponent"](window.location.pathname.toString().substring(0, window.location.pathname.toString().lastIndexOf("/")) + "/")
        } else if (typeof location !== "undefined") {
            PACKAGE_PATH = encodeURIComponent(location.pathname.toString().substring(0, location.pathname.toString().lastIndexOf("/")) + "/")
        } else {
        }
        var PACKAGE_NAME = "python-dateutil.data";
        var REMOTE_PACKAGE_BASE = "python-dateutil.data";
        if (typeof Module["locateFilePackage"] === "function" && !Module["locateFile"]) {
            Module["locateFile"] = Module["locateFilePackage"];
            err("warning: you defined Module.locateFilePackage, that has been renamed to Module.locateFile (using your locateFilePackage for now)")
        }
        var REMOTE_PACKAGE_NAME = Module["locateFile"] ? Module["locateFile"](REMOTE_PACKAGE_BASE, "") : REMOTE_PACKAGE_BASE;
        var REMOTE_PACKAGE_SIZE = metadata.remote_package_size;
        var PACKAGE_UUID = metadata.package_uuid;

        function fetchRemotePackage(packageName, packageSize, callback, errback) {
            var buffer = readFileAsync(packageName);
            callback(buffer);
        }

        function handleError(error) {
            console.error("package error:", error)
        }
        var fetchedCallback = null;
        var fetched = Module["getPreloadedPackage"] ? Module["getPreloadedPackage"](REMOTE_PACKAGE_NAME, REMOTE_PACKAGE_SIZE) : null;
        if (!fetched) fetchRemotePackage(REMOTE_PACKAGE_NAME, REMOTE_PACKAGE_SIZE, function(data) {
            if (fetchedCallback) {
                fetchedCallback(data);
                fetchedCallback = null
            } else {
                fetched = data
            }
        }, handleError);

        function runWithFS() {
            function assert(check, msg) {
                if (!check) throw msg + (new Error).stack
            }
            Module["FS_createPath"]("/", "lib", true, true);
            Module["FS_createPath"]("/lib", "python3.7", true, true);
            Module["FS_createPath"]("/lib/python3.7", "site-packages", true, true);
            Module["FS_createPath"]("/lib/python3.7/site-packages", "python_dateutil-2.7.2-py3.7.egg-info", true, true);
            Module["FS_createPath"]("/lib/python3.7/site-packages", "dateutil", true, true);
            Module["FS_createPath"]("/lib/python3.7/site-packages/dateutil", "zoneinfo", true, true);
            Module["FS_createPath"]("/lib/python3.7/site-packages/dateutil", "parser", true, true);
            Module["FS_createPath"]("/lib/python3.7/site-packages/dateutil", "tz", true, true);

            function DataRequest(start, end, audio) {
                this.start = start;
                this.end = end;
                this.audio = audio
            }
            DataRequest.prototype = {
                requests: {},
                open: function(mode, name) {
                    this.name = name;
                    this.requests[name] = this;
                    Module["addRunDependency"]("fp " + this.name)
                },
                send: function() {},
                onload: function() {
                    var byteArray = this.byteArray.subarray(this.start, this.end);
                    this.finish(byteArray)
                },
                finish: function(byteArray) {
                    var that = this;
                    Module["FS_createPreloadedFile"](this.name, null, byteArray, true, true, function() {
                        Module["removeRunDependency"]("fp " + that.name)
                    }, function() {
                        if (that.audio) {
                            Module["removeRunDependency"]("fp " + that.name)
                        } else {
                            err("Preloading file " + that.name + " failed")
                        }
                    }, false, true);
                    this.requests[this.name] = null
                }
            };

            function processPackageData(arrayBuffer) {
                Module.finishedDataFileDownloads++;
                assert(arrayBuffer, "Loading data file failed.");
                assert(arrayBuffer instanceof ArrayBuffer, "bad input to processPackageData");
                var byteArray = new Uint8Array(arrayBuffer);
                var curr;
                var compressedData = {
                    data: null,
                    cachedOffset: 278800,
                    cachedIndexes: [-1, -1],
                    cachedChunks: [null, null],
                    offsets: [0, 1149, 2223, 3508, 4711, 5968, 7326, 8289, 9242, 10344, 11627, 12884, 13788, 15019, 15990, 16838, 17703, 18344, 19442, 20483, 21420, 22229, 23011, 23807, 24871, 26197, 27399, 28239, 29175, 30154, 31208, 32141, 33066, 34051, 35023, 35832, 36566, 37664, 39099, 40414, 41838, 43109, 44028, 45072, 45847, 46879, 47467, 48427, 49139, 49735, 50551, 51478, 52864, 54147, 55545, 56683, 58731, 60779, 62827, 64875, 66923, 68971, 71019, 73067, 75115, 77163, 79211, 81268, 83316, 85364, 87420, 89470, 91518, 93566, 95616, 97664, 99716, 101689, 103737, 105768, 107816, 109864, 111921, 113969, 116017, 118065, 120122, 122170, 124218, 126275, 128323, 130371, 132419, 134467, 136518, 138566, 140619, 142669, 144717, 146765, 148813, 150861, 152917, 154965, 157013, 159061, 161109, 163157, 165205, 167253, 169301, 171349, 173397, 175445, 177485, 179533, 181581, 183629, 185686, 187734, 189791, 191839, 193887, 195882, 197455, 198740, 199441, 200493, 201740, 202656, 203665, 204552, 205620, 206812, 207952, 209105, 210052, 210901, 212032, 213033, 213966, 215147, 216395, 217427, 218673, 219993, 221285, 222287, 223027, 223921, 224814, 225934, 227249, 228534, 229538, 230446, 231632, 232674, 233684, 234984, 236205, 237433, 238472, 239637, 240648, 241978, 243210, 244283, 245503, 246599, 248083, 249297, 250445, 251582, 252569, 253716, 255004, 256030, 257264, 258425, 259465, 260673, 261794, 263057, 264200, 265163, 266498, 267738, 268836, 270026, 271234, 272273, 273052, 273831, 274828, 275735, 276837, 278178],
                    sizes: [1149, 1074, 1285, 1203, 1257, 1358, 963, 953, 1102, 1283, 1257, 904, 1231, 971, 848, 865, 641, 1098, 1041, 937, 809, 782, 796, 1064, 1326, 1202, 840, 936, 979, 1054, 933, 925, 985, 972, 809, 734, 1098, 1435, 1315, 1424, 1271, 919, 1044, 775, 1032, 588, 960, 712, 596, 816, 927, 1386, 1283, 1398, 1138, 2048, 2048, 2048, 2048, 2048, 2048, 2048, 2048, 2048, 2048, 2048, 2057, 2048, 2048, 2056, 2050, 2048, 2048, 2050, 2048, 2052, 1973, 2048, 2031, 2048, 2048, 2057, 2048, 2048, 2048, 2057, 2048, 2048, 2057, 2048, 2048, 2048, 2048, 2051, 2048, 2053, 2050, 2048, 2048, 2048, 2048, 2056, 2048, 2048, 2048, 2048, 2048, 2048, 2048, 2048, 2048, 2048, 2048, 2040, 2048, 2048, 2048, 2057, 2048, 2057, 2048, 2048, 1995, 1573, 1285, 701, 1052, 1247, 916, 1009, 887, 1068, 1192, 1140, 1153, 947, 849, 1131, 1001, 933, 1181, 1248, 1032, 1246, 1320, 1292, 1002, 740, 894, 893, 1120, 1315, 1285, 1004, 908, 1186, 1042, 1010, 1300, 1221, 1228, 1039, 1165, 1011, 1330, 1232, 1073, 1220, 1096, 1484, 1214, 1148, 1137, 987, 1147, 1288, 1026, 1234, 1161, 1040, 1208, 1121, 1263, 1143, 963, 1335, 1240, 1098, 1190, 1208, 1039, 779, 779, 997, 907, 1102, 1341, 622],
                    successes: [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 1, 0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]
                };
                compressedData.dataName = "python-dateutil.data";
                compressedData.data = byteArray;
                assert(typeof Module.LZ4 === "object", "LZ4 not present - was your app build with  -s LZ4=1  ?");
                Module.LZ4.loadPackage({
                    metadata: metadata,
                    compressedData: compressedData
                });
                Module["removeRunDependency"]("datafile_python-dateutil.data")
            }
            Module["addRunDependency"]("datafile_python-dateutil.data");
            if (!Module.preloadResults) Module.preloadResults = {};
            Module.preloadResults[PACKAGE_NAME] = {
                fromCache: false
            };
            if (fetched) {
                processPackageData(fetched);
                fetched = null
            } else {
                fetchedCallback = processPackageData
            }
        }
        if (Module["calledRun"]) {
            runWithFS()
        } else {
            if (!Module["preRun"]) Module["preRun"] = [];
            Module["preRun"].push(runWithFS)
        }
    };
    loadPackage({
        files: [{
            filename: "/lib/python3.7/site-packages/python_dateutil-2.7.2-py3.7.egg-info/SOURCES.txt",
            start: 0,
            end: 1585,
            audio: 0
        }, {
            filename: "/lib/python3.7/site-packages/python_dateutil-2.7.2-py3.7.egg-info/PKG-INFO",
            start: 1585,
            end: 9508,
            audio: 0
        }, {
            filename: "/lib/python3.7/site-packages/python_dateutil-2.7.2-py3.7.egg-info/dependency_links.txt",
            start: 9508,
            end: 9509,
            audio: 0
        }, {
            filename: "/lib/python3.7/site-packages/python_dateutil-2.7.2-py3.7.egg-info/zip-safe",
            start: 9509,
            end: 9510,
            audio: 0
        }, {
            filename: "/lib/python3.7/site-packages/python_dateutil-2.7.2-py3.7.egg-info/top_level.txt",
            start: 9510,
            end: 9519,
            audio: 0
        }, {
            filename: "/lib/python3.7/site-packages/python_dateutil-2.7.2-py3.7.egg-info/requires.txt",
            start: 9519,
            end: 9528,
            audio: 0
        }, {
            filename: "/lib/python3.7/site-packages/dateutil/rrule.py",
            start: 9528,
            end: 74395,
            audio: 0
        }, {
            filename: "/lib/python3.7/site-packages/dateutil/_common.py",
            start: 74395,
            end: 75327,
            audio: 0
        }, {
            filename: "/lib/python3.7/site-packages/dateutil/easter.py",
            start: 75327,
            end: 78011,
            audio: 0
        }, {
            filename: "/lib/python3.7/site-packages/dateutil/utils.py",
            start: 78011,
            end: 79852,
            audio: 0
        }, {
            filename: "/lib/python3.7/site-packages/dateutil/tzwin.py",
            start: 79852,
            end: 79911,
            audio: 0
        }, {
            filename: "/lib/python3.7/site-packages/dateutil/relativedelta.py",
            start: 79911,
            end: 104404,
            audio: 0
        }, {
            filename: "/lib/python3.7/site-packages/dateutil/_version.py",
            start: 104404,
            end: 104520,
            audio: 0
        }, {
            filename: "/lib/python3.7/site-packages/dateutil/__init__.py",
            start: 104520,
            end: 104742,
            audio: 0
        }, {
            filename: "/lib/python3.7/site-packages/dateutil/zoneinfo/rebuild.py",
            start: 104742,
            end: 106461,
            audio: 0
        }, {
            filename: "/lib/python3.7/site-packages/dateutil/zoneinfo/__init__.py",
            start: 106461,
            end: 112350,
            audio: 0
        }, {
            filename: "/lib/python3.7/site-packages/dateutil/zoneinfo/dateutil-zoneinfo.tar.gz",
            start: 112350,
            end: 251430,
            audio: 0
        }, {
            filename: "/lib/python3.7/site-packages/dateutil/parser/_parser.py",
            start: 251430,
            end: 307188,
            audio: 0
        }, {
            filename: "/lib/python3.7/site-packages/dateutil/parser/__init__.py",
            start: 307188,
            end: 308915,
            audio: 0
        }, {
            filename: "/lib/python3.7/site-packages/dateutil/parser/isoparser.py",
            start: 308915,
            end: 321760,
            audio: 0
        }, {
            filename: "/lib/python3.7/site-packages/dateutil/tz/_factories.py",
            start: 321760,
            end: 323194,
            audio: 0
        }, {
            filename: "/lib/python3.7/site-packages/dateutil/tz/_common.py",
            start: 323194,
            end: 336086,
            audio: 0
        }, {
            filename: "/lib/python3.7/site-packages/dateutil/tz/win.py",
            start: 336086,
            end: 347404,
            audio: 0
        }, {
            filename: "/lib/python3.7/site-packages/dateutil/tz/tz.py",
            start: 347404,
            end: 403784,
            audio: 0
        }, {
            filename: "/lib/python3.7/site-packages/dateutil/tz/__init__.py",
            start: 403784,
            end: 404287,
            audio: 0
        }],
        remote_package_size: 282896,
        package_uuid: "c6e35f06-8ad2-4a34-941c-d86d19fb4876"
    })
})();