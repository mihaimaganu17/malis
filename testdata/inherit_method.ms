class Ciambella {
    cuciniamo() {
        print "Friggere fino a doratura.";
    }
}

class Calabria < Ciambella {
    // Calabria specific stuff
}

Calabria().cuciniamo();

class Campania < Ciambella {
    // Campania specific stuff
    cuciniamo() {
        print "Friggere fino a doratura e rubarlo.";
    }
}

Campania().cuciniamo();
