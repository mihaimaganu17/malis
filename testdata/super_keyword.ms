class AlfaRomeo {
    drive() {
        print "Nice!";
    }
}

class Giulia < AlfaRomeo {
    drive() {
        super.drive();
        print "And comfortable!";

        var method = super.drive;
        method();
    }
}

Giulia.drive();
