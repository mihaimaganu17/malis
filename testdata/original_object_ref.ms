class Persoana {
    striga() {
        print self.name + "!!!";
    }
}

var rodica = Persoana();
rodica.nume = "Rodica";
var metoda = rodica.striga;
metoda();

var ion = Persoana();
ion.nume = "Ion";

ion.striga = rodica.striga;
ion.striga();
