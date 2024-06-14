#include <iostream>
#include <vector>
#include <string>
#include <algorithm>
#include <fstream>
#include <cctype>
#include <iterator>
#include <list>

class List {
private:
    std::vector<std::string> items;

public:
    bool contains(const std::string& item) const {
        return std::find(items.begin(), items.end(), item) != items.end();
    }

    std::vector<std::string>::iterator begin() {
        return items.begin();
    }

    std::vector<std::string>::iterator end() {
        return items.end();
    }

    size_t length() const {
        return items.size();
    }

    void append(const std::string& item, bool front = false) {
        std::vector<std::string> variants;
        variants.push_back(item);
        variants.push_back(toLower(item));
        variants.push_back(toTitle(item));
        variants.push_back(toUpper(item));

        for (size_t i = 0; i < variants.size(); ++i) {
            if (contains(variants[i])) continue;

            if (front) {
                items.insert(items.begin(), variants[i]);
            } else {
                items.push_back(variants[i]);
            }
        }
    }

private:
    static std::string toLower(const std::string& str) {
        std::string result = str;
        std::transform(result.begin(), result.end(), result.begin(), ::tolower);
        return result;
    }

    static std::string toTitle(const std::string& str) {
        std::string result = str;
        if (!result.empty()) {
            result[0] = std::toupper(result[0]);
        }
        return result;
    }

    static std::string toUpper(const std::string& str) {
        std::string result = str;
        std::transform(result.begin(), result.end(), result.begin(), ::toupper);
        return result;
    }
};

class PassGen {
private:
    std::vector<std::string> words;
    std::vector<std::string> b_days;
    bool is_alive;
    List password_list;
    std::vector<std::string> suffix;

public:
    PassGen() : is_alive(true) {
        for (int i = 0; i < 124; ++i) {
            suffix.push_back(std::to_string(i));
        }
    }

    void get_input() {
        while (is_alive) {
            std::cout << "Enter a keyword, name, password, number, symbol, or birthday(mm-dd-yyyy)" << std::endl;
            std::cout << "To generate a password list enter generate" << std::endl;

            std::string user_input;
            std::cout << "\n$> ";
            std::getline(std::cin, user_input);
            user_input = trim(user_input);

            if (!is_alive || user_input.empty()) {
                continue;
            }

            if (toLower(user_input) != "generate") {
                append_data(user_input);
            } else {
                generate();
                is_alive = false;
                continue;
            }

            std::cout << "\n";
        }
    }

private:
    static std::string trim(const std::string& str) {
        size_t first = str.find_first_not_of(' ');
        if (std::string::npos == first) {
            return str;
        }
        size_t last = str.find_last_not_of(' ');
        return str.substr(first, last - first + 1);
    }

    static std::string toLower(const std::string& str) {
        std::string result = str;
        std::transform(result.begin(), result.end(), result.begin(), ::tolower);
        return result;
    }

    void append_data(const std::string& data) {
        if (std::count(data.begin(), data.end(), '-') == 2) {  // birthday
            if (!contains(b_days, data)) {
                b_days.push_back(data);
            }
        } else if (std::all_of(data.begin(), data.end(), ::isdigit)) {  // number
            if (!contains(suffix, data)) {
                suffix.insert(suffix.begin(), data);
            }
            password_list.append(data, true);
        } else if (std::count_if(data.begin(), data.end(), ::isdigit) == (data.size() - 1)) {  // float
            if (!contains(suffix, data)) {
                suffix.insert(suffix.begin(), data);
                suffix.insert(suffix.begin(), filter_digits(data));
            }
            password_list.append(data, true);
            password_list.append(filter_digits(data), true);
        } else if (std::all_of(data.begin(), data.end(), ::isalpha)) {  // words
            if (!contains(words, toLower(data))) {
                words.push_back(data);
            }
        } else if (isSymbol(data)) {  // symbol
            if (!contains(suffix, data)) {
                suffix.insert(suffix.begin(), data);
            }
        } else {  // password
            password_list.append(data, true);
        }
    }

    static bool isSymbol(const std::string& str) {
        return std::all_of(str.begin(), str.end(), [](char c) { return !std::isalpha(c) && !std::isdigit(c); });
    }

    static bool contains(const std::vector<std::string>& vec, const std::string& item) {
        return std::find(vec.begin(), vec.end(), item) != vec.end();
    }

    static std::string filter_digits(const std::string& str) {
        std::string result;
        std::copy_if(str.begin(), str.end(), std::back_inserter(result), ::isdigit);
        return result;
    }

    void generate() {
        std::cout << "Generating a list, this might take a while " << std::endl;
        for (size_t i = 0; i < suffix.size(); ++i) {
            const std::string& suffix_str = suffix[i];
            for (size_t j = 0; j < words.size(); ++j) {
                const std::string& word = words[j];
                password_list.append(word);
                password_list.append(word + suffix_str);
                password_list.append(suffix_str + word);
                password_list.append(suffix_str + word + suffix_str);

                for (size_t k = 0; k < b_days.size(); ++k) {
                    const std::string& bday = b_days[k];
                    std::string day = bday.substr(3, 2);
                    std::string month = bday.substr(0, 2);
                    std::string year = bday.substr(6, 4);
                    std::string plain_bday = bday.substr(0, 2) + bday.substr(3, 2) + bday.substr(6, 4);

                    password_list.append(plain_bday);
                    password_list.append(word + year);
                    password_list.append(word + year.substr(2, 2));
                    password_list.append(word + plain_bday);

                    password_list.append(day + word);
                    password_list.append(day.substr(1, 1) + word);

                    password_list.append(year + word);
                    password_list.append(year.substr(2, 2) + word);

                    password_list.append(month + word);
                    password_list.append(month.substr(1, 1) + word);

                    password_list.append(month + day + word);
                    password_list.append(month.substr(1, 1) + day + word);
                    password_list.append(month + day.substr(1, 1) + word);
                    password_list.append(month.substr(1, 1) + day.substr(1, 1) + word);

                    password_list.append(day + month + word);
                    password_list.append(day.substr(1, 1) + month + word);
                    password_list.append(day + month.substr(1, 1) + word);
                    password_list.append(day.substr(1, 1) + month.substr(1, 1) + word);

                    password_list.append(month + day + word + year);
                    password_list.append(month + day + word + year.substr(2, 2));

                    password_list.append(month.substr(1, 1) + day + word + year);
                    password_list.append(month.substr(1, 1) + day + word + year.substr(2, 2));

                    password_list.append(month + day.substr(1, 1) + word + year);
                    password_list.append(month + day.substr(1, 1) + word + year.substr(2, 2));

                    password_list.append(month.substr(1, 1) + day.substr(1, 1) + word + year);
                    password_list.append(month.substr(1, 1) + day.substr(1, 1) + word + year.substr(2, 2));

                    password_list.append(month + word + suffix_str);
                    password_list.append(month.substr(1, 1) + word + suffix_str);

                    password_list.append(day + word + suffix_str);
                    password_list.append(day.substr(1, 1) + word + suffix_str);

                    password_list.append(suffix_str + word + month);
                    password_list.append(suffix_str + word + month.substr(1, 1));

                    password_list.append(suffix_str + word + day);
                    password_list.append(suffix_str + word + day.substr(1, 1));

                    password_list.append(suffix_str + word + year);
                    password_list.append(suffix_str + word + year.substr(2, 2));
                }
            }
        }

        std::ofstream output_file("passwords.txt");
        if (output_file.is_open()) {
            std::cout << "Generated a list of " << password_list.length() << " passwords ..." << std::endl;

            for (auto it = password_list.begin(); it != password_list.end(); ++it) {
                output_file << *it << "\n";
            }
            output_file.close();
        }
    }
};

int main() {
    PassGen().get_input();
    return 0;
}

