class AlfaRomeo {
    drive() {
        print "Nice!";
    }
}

class Giulia < AlfaRomeo {
    drive() {
        super.drive();
        print "And comfortable!";
    }
}

Giulia.drive();
