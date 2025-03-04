class Omleta {
    umpluta_cu(umplutura) {
        return "Omleata umpluta cu " + umplutura;
    }
}

fun omleta(proteina, carne) {
    print "Mic dejun cu " + proteina + " si " + carne;
}

var omleta = Omleta();
var oua = omleta.umpluta_cu("branza");
mic_dejun(oua, "carnat");
