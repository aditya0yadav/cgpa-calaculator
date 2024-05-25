#include <iostream>
#include <string>
using namespace std;

struct Student {
    string name;
    int physics;
    int chemistry;
    int maths;
};

int main() {
    Student marks;
    cin >> marks.name;
    cin >> marks.physics;
    cin >> marks.chemistry;
    cin >> marks.maths;

    int tMarks = marks.physics + marks.chemistry + marks.maths;
    int totalMarks = tMarks / 3;

    if (totalMarks >= 90) {
        cout << "A";
    } else if (totalMarks >= 80) {
        cout << "B";
    } else if (totalMarks >= 70) {
        cout << "C";
    } else if (totalMarks >= 60) {
        cout << "D";
    } else if (totalMarks >= 40) {
        cout << "E";
    } else {
        cout << "F";
    }
    return 0;
}
