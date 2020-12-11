#include <iostream>
#include <fstream>
#include <vector>
#include <string>
#include <unordered_set>

using namespace std;
void getMultipliedValue( int left, int right ){
    cout << left << " | " << right << endl;
    cout << left*right << endl;
}
void getSum2( vector<int> &values ){
    vector<int>::const_iterator left = values.begin();
    vector<int>::const_iterator right = --values.end();
    cout << *left << endl;
    cout << *right << endl;
    int total = 2020;
    while( left < right ){
        int sum = *left + *right;
        if( sum == total ){
            getMultipliedValue(*left, *right);
            break;
        }else if( sum < total ){
            left++;
        }else{
            right--;
        }
    }
}
void getSum3( vector<int>& values ){
    unordered_set< int > seen;
    // Be careful with values that can be doubled up    
    for( int i = 1; i < values.size() ; i++ ){
        if( values[i] != values[i-1] ){
            for( int j = i + 1; j < values.size(); j++ ){
                int complement = 2020 - values[i] - values[j];
                if( seen.count( complement ) ){
                    cout << values[i] << " | " << values[j] << " | " << complement << endl;
                    cout << values[i]*values[j]*complement << endl;
                    return;
                }
                seen.insert( values[j] );
            }
            seen.clear();
        } 
     
    }
}
int main( int argc, char* argv[] ){
    if( argc < 2 ){
        cout << "Please provide an input:" << endl;
        cout << "\tReportRepair <inputFile>" << endl;
    }
    ifstream inputFile( argv[1] );
    if( !inputFile ){
        cerr << "Couldn't open file: " << argv[1] << endl;
        return EXIT_FAILURE;
    }
    int value;
    vector<int> values;
    while( inputFile >> value ) {
        values.push_back( value ); 
    }
    
    sort( values.begin(), values.end() );
    //for( int v : values ){
    //    cout << v << endl;
    //}
    getSum3( values );
    inputFile.close();
    return EXIT_SUCCESS;
}
